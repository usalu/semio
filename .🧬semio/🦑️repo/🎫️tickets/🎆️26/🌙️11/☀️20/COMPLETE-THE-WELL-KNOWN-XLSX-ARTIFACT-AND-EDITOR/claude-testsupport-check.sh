#!/bin/zsh
# [DEBUG] wasm32-wasip2 cargo check of the store test-support crate only
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️20/COMPLETE-THE-WELL-KNOWN-XLSX-ARTIFACT-AND-EDITOR"; N=$T/🗑️generated/claude-pane/native
export CARGO_BUILD_RUSTC=rustc CARGO_BUILD_RUSTC_WRAPPER= CARGO_TARGET_DIR=$N/target CARGO_BUILD_BUILD_DIR=$N/build CARGO_BUILD_JOBS=4 CARGO_INCREMENTAL=0
export CARGO_UNSTABLE_FEATURE_UNIFICATION=true CARGO_RESOLVER_FEATURE_UNIFICATION=package
export CARGO_ENCODED_RUSTFLAGS=$(printf -- "-Z\x1fthreads=1\x1f-Z\x1fshare-generics=y")
cd /Users/ueli/Documents/semio/🌎️hub || exit 1
cargo check -j 4 --target wasm32-wasip2 --profile dev -p semio-os-store-test-support
