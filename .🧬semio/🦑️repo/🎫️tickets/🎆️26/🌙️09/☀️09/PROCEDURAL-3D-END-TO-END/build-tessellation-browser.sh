#!/bin/sh
set -eu
repo_root=$(pwd)
ticket=$(CDPATH='' cd "$(dirname "$0")" && pwd)
area="$ticket/🗑️generated/bridge-current"
src='✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep'
rust_bin=$(rustup which rustc)
export RUSTC="$rust_bin" RUSTC_WRAPPER="$ticket/rebuild-procedural-requester.py" SEMIO_PINNED_LINK_CRATE=semio_s_plugin_flow_brep_component RUST_MIN_STACK=67108864 CARGO_HOME="$repo_root/.cache/b/r/m/cargo/home" CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 RUSTFLAGS="-Zthreads=1 -Clink-arg=--max-memory=536870912 -Clink-arg=-zstack-size=8388608"
export CARGO_TARGET_DIR="${SEMIO_BREP_TARGET_DIR:-$area/batched-brep}" CARGO_BUILD_BUILD_DIR="$repo_root/.cache/b/r/m/tmp/g3d-brep-stable-n25"
cd "$repo_root/.cache/g3d-native/b"
exec "$(dirname "$RUSTC")/cargo" build --offline -j1 --target wasm32-wasip2 --manifest-path "$src/🧩️component/📦️packages/🦀️rust/Cargo.toml"
