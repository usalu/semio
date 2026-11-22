#!/bin/zsh
# [DEBUG] native wasm32-wasip2 build of the standalone XLSX pane boot components (stdio, stdio-office)
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️20/COMPLETE-THE-WELL-KNOWN-XLSX-ARTIFACT-AND-EDITOR"; N=$T/🗑️generated/claude-pane/native
mkdir -p $N
export CARGO_BUILD_RUSTC=rustc CARGO_BUILD_RUSTC_WRAPPER= CARGO_TARGET_DIR=$N/target CARGO_BUILD_BUILD_DIR=$N/build
export CARGO_BUILD_JOBS=${JOBS:-4} CARGO_PROFILE_DEV_DEBUG=0 CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_SPLIT_DEBUGINFO=off
export CARGO_UNSTABLE_FEATURE_UNIFICATION=true CARGO_RESOLVER_FEATURE_UNIFICATION=package
cd /Users/ueli/Documents/semio/🌎️hub || exit 1
cargo build -p semio-hub-stdio-component -p semio-hub-stdio-office-component --lib --target wasm32-wasip2 --profile wasm-prototype \
 --config 'profile.wasm-prototype.inherits="dev"' --config 'profile.wasm-prototype.opt-level="s"' --config 'profile.wasm-prototype.codegen-units=16' --config 'profile.wasm-prototype.lto="thin"' --config 'profile.wasm-prototype.incremental=false' "$@"
echo "exit $?"
