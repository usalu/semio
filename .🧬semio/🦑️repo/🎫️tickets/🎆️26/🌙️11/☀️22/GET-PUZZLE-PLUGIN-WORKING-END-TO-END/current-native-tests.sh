#!/usr/bin/env bash
set -eu
root=$PWD
output="$(cd "$(dirname "$0")" && pwd)/🗑️generated"
toolchain="$root/.cache/b/t/m/r/699c2d5d27a9c904/toolchains/nightly-2026-07-20-x86_64-unknown-linux-gnu/bin"
export CARGO_HOME="$root/.cache/b/r/m/cargo/home" ZIG_GLOBAL_CACHE_DIR="$output/zig" ZIG_LOCAL_CACHE_DIR="$output/zig-local"
export RUSTC="$toolchain/rustc" RUSTC_WRAPPER="" CARGO_BUILD_BUILD_DIR="$output/current-native/b" CARGO_TARGET_DIR="$output/current-native/t"
export CARGO_PROFILE_TEST_DEBUG=0 CARGO_INCREMENTAL=0 RUSTFLAGS="-Z threads=1"
cd "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/$1"
shift
exec "$toolchain/cargo" test --locked --offline -j2 "$@"
