#![cfg(not(miri))] // Miri does not support custom allocators

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL: jemallocator::Jemalloc = jemallocator::Jemalloc;

#[cfg(target_os = "windows")]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

use std::path::PathBuf;

use oxc_cli::{Cli, Command};

fn main() {
    match Command::new().build().get_matches().subcommand() {
        Some(("lint", matches)) => {
            let path = matches.get_one::<PathBuf>("path").unwrap();
            Cli::lint(path);
        }
        _ => unreachable!(),
    }
}
