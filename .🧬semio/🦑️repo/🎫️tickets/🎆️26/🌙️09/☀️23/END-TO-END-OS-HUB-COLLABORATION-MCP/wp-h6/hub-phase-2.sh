#!/bin/zsh
# H6 hub-mutex phase 2 (after the peer dwg fix): hub test quick + long, full browser-document-open-check.
W=/Users/ueli/Documents/semio/.tmp-ticket/wp-h6; G=$W/generated
export CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=$W/target RUST_MIN_STACK=268435456
cd /Users/ueli/Documents/semio
bun nx run os-hub:test-quick --skip-nx-cache -- --no-fail-fast > $G/hub-quick-2.txt 2>&1; echo "QUICK rc=$? $(date +%T)"
bun nx run os-hub:test-long --skip-nx-cache > $G/hub-long-2.txt 2>&1; echo "LONG rc=$? $(date +%T)"
bun nx run os-hub:browser-document-open-check --skip-nx-cache > $G/browser-document-open-check-2.txt 2>&1; echo "CHAIN rc=$? $(date +%T)"
echo "PHASE DONE $(date +%T)"
