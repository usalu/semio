#!/bin/zsh
# 🦀️ One niced cargo run on the H11 private target, full capture in the durable log dir, summary on stdout.
# usage: h11-cargo.sh <label> <cargo args…>
cd /Users/ueli/Documents/semio
LABEL=$1; shift
LOG="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s13-h11-logs/$LABEL.txt"
export CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.tmp-ticket/wp-h11/target RUST_MIN_STACK=268435456
export CARGO_BUILD_BUILD_DIR=${H11_BUILD_DIR:-/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b}
export SEMIO_TEST_ARTIFACT_DIR="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s13-h11-test-artifacts/$LABEL"
mkdir -p "$SEMIO_TEST_ARTIFACT_DIR" && chmod 700 "$SEMIO_TEST_ARTIFACT_DIR"
echo "=== start $(date +%T) $LABEL: cargo $*" | tee "$LOG"
nice -n 10 cargo "$@" >> "$LOG" 2>&1
rc=$?
echo "EXIT $rc $(date +%T)" | tee -a "$LOG"
/usr/bin/grep -E '^error(\[|:)|test result|^test .* FAILED|panicked at|Summary|^        (FAIL|TIMEOUT|SIGABRT|SIGSEGV)' "$LOG" | head -60
