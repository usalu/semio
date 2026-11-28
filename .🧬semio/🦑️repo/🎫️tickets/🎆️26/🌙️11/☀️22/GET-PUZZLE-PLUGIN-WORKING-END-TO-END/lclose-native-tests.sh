#!/usr/bin/env bash
set -eu
root=$PWD
output="$(cd "$(dirname "$0")" && pwd)/🗑️generated"
toolchain="$root/.cache/b/t/m/r/699c2d5d27a9c904/toolchains/nightly-2026-07-20-x86_64-unknown-linux-gnu/bin"
export CARGO_HOME="$root/.cache/b/r/m/cargo/home" ZIG_GLOBAL_CACHE_DIR="$output/zig" ZIG_LOCAL_CACHE_DIR="$output/zig-local"
export RUSTC="$toolchain/rustc" RUSTC_WRAPPER="" CARGO_BUILD_BUILD_DIR="$output/lclose-native/b" CARGO_TARGET_DIR="$output/lclose-native/t"
export RUST_MIN_STACK=134217728
export CARGO_PROFILE_TEST_DEBUG=0 CARGO_INCREMENTAL=0 RUSTFLAGS="-Z threads=1"
cd "$1"
shift
resolution=(--offline --locked)
if [[ ${SEMIO_NATIVE_REFRESH_LOCK:-0} == 1 ]]; then resolution=(--offline); fi
exec "$toolchain/cargo" test "${resolution[@]}" -j "${SEMIO_NATIVE_CARGO_JOBS:-2}" "$@"
