#!/usr/bin/env bash
set -eu
project=$1
shift
output="$(cd "$(dirname "$0")" && pwd)/🗑️generated"
toolchain="$PWD/.cache/b/t/m/r/699c2d5d27a9c904/toolchains/nightly-2026-07-20-x86_64-unknown-linux-gnu/bin"
export CARGO_HOME="$PWD/.cache/b/r/m/cargo/home" RUSTC="$toolchain/rustc" RUSTC_WRAPPER=""
export CARGO_BUILD_BUILD_DIR="$output/current-bytes-native/b" CARGO_TARGET_DIR="$output/current-bytes-native/t"
export RUSTFLAGS="-Z threads=1" CARGO_INCREMENTAL=0 CARGO_PROFILE_TEST_DEBUG=0 RUST_MIN_STACK=134217728
export ZIG_GLOBAL_CACHE_DIR="$output/zig" ZIG_LOCAL_CACHE_DIR="$output/zig-local"
cd "$project"
lock=(--locked)
if [[ ${SEMIO_REFRESH_FRAMEWORK_LOCK:-0} == 1 ]]; then lock=(); fi
exec "$toolchain/cargo" test "${lock[@]}" --offline -j2 "$@"
