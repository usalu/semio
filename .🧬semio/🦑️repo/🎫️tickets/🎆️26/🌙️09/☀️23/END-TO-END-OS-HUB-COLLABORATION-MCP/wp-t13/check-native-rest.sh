#!/bin/zsh
# 🧪️ T13: native check of the crates the first gate invocation did not reach (wfc-2d without features, surface, wires).
cd /Users/ueli/Documents/semio || exit 1
export CARGO_BUILD_BUILD_DIR=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.tmp-ticket/wp-t13/target CARGO_INCREMENTAL=0
nice -n 10 cargo check --keep-going -p semio-s-artifact-wfc-2d -p semio-s-artifact-wfc-grid2d -p semio-s-artifact-wfc-bitmap -p semio-s-artifact-wfc-3d -p semio-s-artifact-wfc-grid3d -p semio-framework-surface -p semio-s-artifact-reasoning-wires --lib --tests --message-format short; echo "EXIT-NATIVE $? $(date '+%T')"
