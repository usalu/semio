#!/bin/zsh
# ✅️ T14 window-3 T1 hotfix gate 2 (native lane, live tree): re-check os/framework/plugin-host (kernel-home locale re-export), then the stdio
# plugin's native_openable_provider test (h9l `name`→`label`).
export CARGO_INCREMENTAL=0 NX_DAEMON=false
export CARGO_BUILD_BUILD_DIR="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b" CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.tmp-ticket/wp-t14/target
cd /Users/ueli/Documents/semio || exit 2
echo "LANE $(date +%T)"
cargo check -p semio-framework-os -p semio-framework -p semio-framework-plugin-host --lib --tests --message-format short 2>&1 | /usr/bin/grep -E ': error|^error|Finished' ; echo "RC-CHECK ${pipestatus[1]} $(date +%T)"
cargo test --no-fail-fast -p semio-s-plugin-stdio --test native_openable_provider 2>&1 | /usr/bin/grep -E ': error|^error|^test |test result|panicked' ; echo "RC ${pipestatus[1]} $(date +%T)"
