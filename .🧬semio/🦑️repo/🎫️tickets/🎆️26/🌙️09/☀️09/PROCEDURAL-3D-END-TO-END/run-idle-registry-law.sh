#!/bin/sh
set -eu
ticket=$(CDPATH='' cd "$(dirname "$0")" && pwd)
repo_root=$(pwd)
rust_bin=$(rustup which rustc)
export RUSTC="$rust_bin" RUSTC_WRAPPER="$ticket/reactor-idle-compiler.py" CARGO_HOME="$repo_root/.cache/b/r/m/cargo/home" RUSTFLAGS="-Zthreads=1" CARGO_PROFILE_DEV_DEBUG=0 CARGO_INCREMENTAL=0 RUST_MIN_STACK=67108864
export CARGO_BUILD_BUILD_DIR="$repo_root/.cache/b/r/m/tmp/flow-native-n25" CARGO_TARGET_DIR="$ticket/🗑️generated/bridge-current/idle-registry-target"
exec "$(dirname "$RUSTC")/cargo" test --offline -j1 --manifest-path "$ticket/🗑️generated/bridge-current/idle-registry-source/Cargo.toml" --lib cold_pair_ingress_is_an_exact_retained_native_close_participant -- --nocapture
