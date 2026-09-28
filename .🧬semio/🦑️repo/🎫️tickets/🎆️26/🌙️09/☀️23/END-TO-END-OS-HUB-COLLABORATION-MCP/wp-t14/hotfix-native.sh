#!/bin/zsh
# ✅️ T14 window-3 T1 hotfix gate (native lane, live tree): semio-framework-os + semio-framework + plugin-host lib/tests check.
export CARGO_INCREMENTAL=0 NX_DAEMON=false
export CARGO_BUILD_BUILD_DIR="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b" CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.tmp-ticket/wp-t14/target
cd /Users/ueli/Documents/semio || exit 2
echo "LANE $(date +%T)"
cargo check -p semio-framework-os -p semio-framework -p semio-framework-plugin-host --lib --tests --message-format short 2>&1 | /usr/bin/grep -E ': error|^error|generated [0-9]+ warning|Finished' ; echo "RC ${pipestatus[1]} $(date +%T)"
cargo check -p semio-framework-os --features os-host-full --lib --tests --message-format short 2>&1 | /usr/bin/grep -E ': error|^error|Finished' ; echo "RC-FULL ${pipestatus[1]} $(date +%T)"
