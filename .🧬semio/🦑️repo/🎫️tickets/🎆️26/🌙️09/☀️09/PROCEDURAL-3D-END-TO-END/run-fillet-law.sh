#!/bin/sh
set -eu
ticket=$(CDPATH='' cd "$(dirname "$0")" && pwd)
repo_root=$(pwd)
rust_bin=$(rustup which rustc)
export RUSTC="$rust_bin" RUSTC_WRAPPER="$ticket/🗑️generated/bridge-current/retry-compiler" CARGO_HOME="$repo_root/.cache/b/r/m/cargo/home" RUSTFLAGS="-Zthreads=1" CARGO_PROFILE_DEV_DEBUG=0 CARGO_INCREMENTAL=0 RUST_MIN_STACK=67108864
export CARGO_BUILD_BUILD_DIR="$repo_root/.cache/b/r/m/tmp/flow-native-n25" CARGO_TARGET_DIR="$ticket/🗑️generated/bridge-current/fillet-law-target"
exec "$(dirname "$RUSTC")/cargo" test --offline -j1 --manifest-path "$repo_root/.cache/g3d-native/s/✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/📐️tessellate/📦️packages/🦀️rust/Cargo.toml" --lib original_browser_stock -- --nocapture
