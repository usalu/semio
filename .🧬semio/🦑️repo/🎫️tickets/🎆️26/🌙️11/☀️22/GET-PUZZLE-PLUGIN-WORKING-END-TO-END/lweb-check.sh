#!/usr/bin/env bash
set -eu
export CARGO_INCREMENTAL=0
root=$PWD
ticket=$(cd "$(dirname "$0")" && pwd)
output="$ticket/🗑️generated"
toolchain="$root/.cache/b/t/m/r/699c2d5d27a9c904/toolchains/nightly-2026-07-20-x86_64-unknown-linux-gnu/bin"
export CARGO_HOME="$root/.cache/b/r/m/cargo/home" ZIG_GLOBAL_CACHE_DIR="$output/zig" ZIG_LOCAL_CACHE_DIR="$output/zig-local"
export RUSTC="$toolchain/rustc" RUSTC_WRAPPER="" CARGO_BUILD_BUILD_DIR="$output/lweb-check/b" CARGO_TARGET_DIR="$output/lweb-check/t"
export PATH="$toolchain:$(dirname "$(command -v repo)"):$PATH"
profile=()
while IFS= read -r setting; do profile+=(--config "$setting"); done < "$output/p4-profile-selectors.txt"
cd "🌎️hub/🧩️compositions/🧩️puzzle/🧩️component/📦️packages/🦀️rust"
"$toolchain/cargo" check --offline --target wasm32-unknown-unknown --jobs 2 -p semio-framework-replication-history -p semio-os-history-schema "${profile[@]}" --profile prototype
