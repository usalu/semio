#!/usr/bin/env bash
set -eu
root=$PWD
ticket=$(cd "$(dirname "$0")" && pwd)
output="$ticket/🗑️generated"
toolchain="$root/.cache/b/t/m/r/699c2d5d27a9c904/toolchains/nightly-2026-07-20-x86_64-unknown-linux-gnu/bin"
export CARGO_HOME="$root/.cache/b/r/m/cargo/home" RUSTC="$toolchain/rustc" RUSTC_WRAPPER="" CARGO_BUILD_BUILD_DIR="$output/current-tick-native/b" CARGO_TARGET_DIR="$output/current-tick-native/t" CARGO_PROFILE_TEST_DEBUG=0 CARGO_INCREMENTAL=0 RUSTFLAGS="-Z threads=1" ZIG_GLOBAL_CACHE_DIR="$output/zig" ZIG_LOCAL_CACHE_DIR="$output/zig-local" RUST_MIN_STACK=134217728
exec "$toolchain/cargo" test --offline --locked -j 2 --manifest-path "$1" --lib
