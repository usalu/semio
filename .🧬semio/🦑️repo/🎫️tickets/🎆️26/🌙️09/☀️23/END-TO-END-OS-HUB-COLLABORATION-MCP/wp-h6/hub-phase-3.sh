#!/bin/zsh
# H6 hub-mutex phase 3: hub test quick + long on the final tree (after the kernel sync fixes).
W=/Users/ueli/Documents/semio/.tmp-ticket/wp-h6; G=$W/generated
export CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=$W/target RUST_MIN_STACK=268435456
cd /Users/ueli/Documents/semio
bun nx run os-hub:test-quick --skip-nx-cache -- --no-fail-fast > $G/hub-quick-3.txt 2>&1; echo "QUICK rc=$? $(date +%T)"
bun nx run os-hub:test-long --skip-nx-cache > $G/hub-long-3.txt 2>&1; echo "LONG rc=$? $(date +%T)"
echo "PHASE DONE $(date +%T)"
