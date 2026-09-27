#!/bin/zsh
# 🦀️ H13: one cargo run on the H13 private target in build-fleet-b (preamble rule 3), full capture in the durable log dir,
# summary on stdout. Called inside a native-lane hold (h13-hold.sh), never on its own.
# usage: h13-cargo.sh <label> <cargo args…>
cd /Users/ueli/Documents/semio
LABEL=$1; shift
LOG="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-h13-logs/$LABEL.txt"
export CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.tmp-ticket/wp-h13/target RUST_MIN_STACK=268435456
export CARGO_BUILD_BUILD_DIR=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b
export SEMIO_TEST_ARTIFACT_DIR="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-h13-test-artifacts/$LABEL"
mkdir -p "$SEMIO_TEST_ARTIFACT_DIR" && chmod 700 "$SEMIO_TEST_ARTIFACT_DIR"
echo "=== start $(date +%T) $LABEL: cargo $*" | tee "$LOG"
nice -n 15 cargo "$@" >> "$LOG" 2>&1
rc=$?
echo "EXIT $rc $(date +%T)" | tee -a "$LOG"
/usr/bin/grep -E '^error(\[|:)|test result|^test .* FAILED|panicked at|^warning: `|Summary|^        (FAIL|TIMEOUT|SIGABRT|SIGSEGV)' "$LOG" | head -60
exit $rc
