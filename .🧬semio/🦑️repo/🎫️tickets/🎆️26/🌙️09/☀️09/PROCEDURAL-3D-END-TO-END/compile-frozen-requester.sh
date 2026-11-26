#!/bin/sh
set -eu
ticket=$(CDPATH='' cd "$(dirname "$0")" && pwd)
root=$(pwd)
area="$ticket/🗑️generated/bridge-current"
work="$root/.cache/g3d-native/p/🌎️hub"
toolchain=$(rustup which rustc)
cd "$work"
export RUSTC="$toolchain" RUSTC_WRAPPER="" CARGO_HOME="$root/.cache/b/r/m/cargo/home" CARGO_INCREMENTAL=0 CARGO_BUILD_BUILD_DIR="$root/.cache/b/r/m/tmp/g3d-frozen-n26" CARGO_TARGET_DIR="$area/frozen-lean"
exec nice -n 8 "$(dirname "$toolchain")/cargo" build --offline --locked -j2 --manifest-path '🧩️compositions/🌀️procedural/🧩️component/📦️packages/🦀️rust/Cargo.toml' --target wasm32-wasip2 --config 'profile.dev.opt-level="s"' --config 'profile.dev.debug=0' --config 'profile.dev.incremental=false' --config 'profile.dev.codegen-units=16' --config 'profile.dev.lto="off"' --config 'profile.dev.strip="symbols"' --config 'profile.dev.build-override.opt-level=0'
