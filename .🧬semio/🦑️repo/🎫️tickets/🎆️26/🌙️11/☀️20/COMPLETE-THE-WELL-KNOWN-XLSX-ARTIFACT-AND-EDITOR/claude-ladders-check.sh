#!/bin/zsh
# [DEBUG] guarded check of the maintenance ladders crate and its tests
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️20/COMPLETE-THE-WELL-KNOWN-XLSX-ARTIFACT-AND-EDITOR"; N=$T/🗑️generated/claude-cleanup-pump
export CARGO_BUILD_RUSTC=rustc CARGO_BUILD_RUSTC_WRAPPER= CARGO_TARGET_DIR=$N/target CARGO_BUILD_BUILD_DIR=$N/build
export CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 CARGO_INCREMENTAL=0
cd /Users/ueli/Documents/semio || exit 1
cargo check -j 2 -p semio-plugin-instance-ladders --tests > $N/ladders-check.log 2>&1; echo "[DEBUG] ladders check exit $?" >> $N/ladders-check.log
