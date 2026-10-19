#!/bin/bash
# Runs one repo-library test file under emulated root declarations (diagnostic): run-lib-test.sh <repo-relative test file> <log>
cd /Users/ueli/Documents/semio
C="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
export SEMIO_TEST_ARTIFACT_DIR="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/🧪️tests/tst1"
mkdir -p "$SEMIO_TEST_ARTIFACT_DIR"
bun test --tsconfig-override "$C/tst1-tools/runtime-tsconfig.json" --timeout 120000 "$1" > "$2" 2>&1
rc=$?
pass=$(/usr/bin/grep -E "^ *[0-9]+ pass" "$2" | tr -s ' ' | cut -d' ' -f2)
fail=$(/usr/bin/grep -E "^ *[0-9]+ fail" "$2" | tr -s ' ' | cut -d' ' -f2)
echo -e "$(basename "$(dirname "$1")")\trc=$rc\tpass=$pass\tfail=$fail"
