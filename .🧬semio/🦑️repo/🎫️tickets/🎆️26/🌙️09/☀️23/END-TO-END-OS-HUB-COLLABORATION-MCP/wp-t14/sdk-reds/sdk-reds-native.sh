#!/bin/zsh
# ✅️ T14 rule-22 gate for `sdk-reds/sdk-lib-reds.py` (live tree, native lane via fleet-mutex): the SDK lib tests compile
# (`--lib --tests` check) and the two fixed laws pass. usage: sdk-reds-native.sh
export CARGO_INCREMENTAL=0 RUST_MIN_STACK=33554432 NX_DAEMON=false
export CARGO_BUILD_BUILD_DIR="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b" CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.tmp-ticket/wp-t14/target
cd /Users/ueli/Documents/semio || exit 2
echo "LANE $(date +%T)"
cargo check -p semio-framework-plugin --lib --tests --message-format short 2>&1 | tail -5; echo "CHECK-RC ${pipestatus[1]} $(date +%T)"
cargo test --no-fail-fast -p semio-framework-plugin --lib -- activated_tool_factory_keys_are_an_exact_bijection_with_migrated_declarations a_scene_that_declares_no_lanes_still_publishes_one_childless_surface 2>&1 | /usr/bin/grep -E '^test |test result|panicked|left|right|error' | head -30; echo "TEST-RC ${pipestatus[1]} $(date +%T)"
