# Platform Toolchain Readiness

Read the active compiler, installed Rust targets and installed components before the remaining platform warning gates. This is toolchain inventory, not a completed platform build.

## rustup target list --installed

aarch64-apple-darwin
wasm32-unknown-unknown
wasm32-wasip1
wasm32-wasip2
x86_64-pc-windows-msvc
x86_64-unknown-linux-gnu

## rustup component list --installed

cargo-aarch64-apple-darwin
clippy-aarch64-apple-darwin
llvm-tools-aarch64-apple-darwin
rust-docs-aarch64-apple-darwin
rust-src
rust-std-aarch64-apple-darwin
rust-std-wasm32-unknown-unknown
rust-std-wasm32-wasip1
rust-std-wasm32-wasip2
rust-std-x86_64-pc-windows-msvc
rust-std-x86_64-unknown-linux-gnu
rustc-aarch64-apple-darwin
rustfmt-aarch64-apple-darwin

## rustc -vV

rustc 1.99.0-nightly (c4af71034 2026-07-06)
binary: rustc
commit-hash: c4af71034e89a431eeee91125a31ad001379faac
commit-date: 2026-07-06
host: aarch64-apple-darwin
release: 1.99.0-nightly
LLVM version: 22.1.8

Missing required targets: none. Clippy installed: true.
