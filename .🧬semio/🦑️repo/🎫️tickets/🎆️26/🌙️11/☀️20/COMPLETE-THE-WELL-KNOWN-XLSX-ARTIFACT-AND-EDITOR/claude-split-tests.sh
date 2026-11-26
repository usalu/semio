#!/bin/zsh
# [DEBUG] native test run of the laws the runtime-host split touches
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️20/COMPLETE-THE-WELL-KNOWN-XLSX-ARTIFACT-AND-EDITOR"; N=$T/🗑️generated/claude-split/native-test
export CARGO_BUILD_RUSTC=rustc CARGO_BUILD_RUSTC_WRAPPER= CARGO_TARGET_DIR=$N/target CARGO_BUILD_BUILD_DIR=$N/build
export CARGO_BUILD_JOBS=4 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 CARGO_INCREMENTAL=0
cd /Users/ueli/Documents/semio/🌎️hub || exit 1
code=0
cargo test -j 4 -p semio-hub-stdio-catalog-projection -p semio-hub-stdio-catalog || code=$?
cargo test -j 4 -p semio-hub-stdio --test shipped_fleet --test native_openable_provider --test component_assembly || code=$?
cargo test -j 4 -p semio-hub-native-openable || code=$?
echo "[DEBUG] tests exit $code"
exit $code
