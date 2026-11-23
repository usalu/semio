#!/bin/zsh
# [DEBUG] separate-target wasm check of the Space component
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️20/COMPLETE-THE-WELL-KNOWN-XLSX-ARTIFACT-AND-EDITOR"; N=$T/🗑️generated/claude-space-migrate
export CARGO_BUILD_RUSTC=rustc CARGO_BUILD_RUSTC_WRAPPER= CARGO_TARGET_DIR=$N/target CARGO_BUILD_BUILD_DIR=$N/build
export CARGO_BUILD_JOBS=4 CARGO_PROFILE_DEV_DEBUG=0 CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_SPLIT_DEBUGINFO=off
export CARGO_UNSTABLE_FEATURE_UNIFICATION=true CARGO_RESOLVER_FEATURE_UNIFICATION=package
cd /Users/ueli/Documents/semio/🌎️hub || exit 1
cargo check -p ${PKG:-semio-hub-space-component} --lib --target wasm32-wasip2 -j 4 "$@" > $N/check.log 2>&1
echo "exit $?" >> $N/check.log
