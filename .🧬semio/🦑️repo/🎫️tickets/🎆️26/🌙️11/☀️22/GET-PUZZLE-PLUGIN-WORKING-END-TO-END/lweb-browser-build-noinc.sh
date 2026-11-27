#!/usr/bin/env bash
set -eu
export CARGO_INCREMENTAL=0
root=$PWD
ticket=$(cd "$(dirname "$0")" && pwd)
output="$ticket/🗑️generated"
toolchain="$root/.cache/b/t/m/r/699c2d5d27a9c904/toolchains/nightly-2026-07-20-x86_64-unknown-linux-gnu/bin"
export CARGO_HOME="$root/.cache/b/r/m/cargo/home" ZIG_GLOBAL_CACHE_DIR="$output/zig" ZIG_LOCAL_CACHE_DIR="$output/zig-local"
export RUSTC="$toolchain/rustc" RUSTC_WRAPPER="" CARGO_BUILD_BUILD_DIR="$output/current-wasm/b" CARGO_TARGET_DIR="$output/current-wasm/t"
export PATH="$toolchain:$(dirname "$(command -v repo)"):$PATH"
profile=()
while IFS= read -r setting; do profile+=(--config "$setting"); done < "$output/p4-profile-selectors.txt"
cd "🌎️hub/🧩️compositions/🧩️puzzle/🧩️component/📦️packages/🦀️rust"
for attempt in 1 2 3 4 5 6; do "$toolchain/cargo" build --offline --profile prototype --target wasm32-unknown-unknown --jobs 1 "${profile[@]}" && break; echo "[DEBUG] build attempt $attempt failed"; done; [ -f "$CARGO_TARGET_DIR/wasm32-unknown-unknown/prototype/semio_hub_puzzle_component.wasm" ]
"$CARGO_HOME/bin/wasm-bindgen" "$CARGO_TARGET_DIR/wasm32-unknown-unknown/prototype/semio_hub_puzzle_component.wasm" --out-dir "$output/current-browser-pkg" --typescript --target web --out-name semio_puzzle
