#!/bin/zsh
# 🧰️ H13: one permanent verb (bun 📜️script.ts …) whose cargo runs on the H13 private target in build-fleet-b (preamble rule 3),
# full capture in the durable log dir. Called inside a native-lane hold (h13-hold.sh), never on its own.
# usage: h13-verb.sh <label> <package dir> <verb args…>
LABEL=$1; DIR=$2; shift 2
LOG="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-h13-logs/$LABEL.txt"
export CARGO_INCREMENTAL=0 CARGO_TARGET_DIR="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-h13-target" RUST_MIN_STACK=268435456 NX_DAEMON=false
export CARGO_BUILD_BUILD_DIR=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b
export SEMIO_TEST_ARTIFACT_DIR="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-h13-test-artifacts/$LABEL"
mkdir -p "$SEMIO_TEST_ARTIFACT_DIR" && chmod 700 "$SEMIO_TEST_ARTIFACT_DIR"
cd "$DIR" || exit 1
echo "=== start $(date +%T) $LABEL: bun ./📜️script.ts $*" | tee "$LOG"
nice -n 15 bun ./📜️script.ts "$@" >> "$LOG" 2>&1
rc=$?
echo "EXIT $rc $(date +%T)" | tee -a "$LOG"
/usr/bin/grep -E '^\[acceptance\]|^\[hostile-input\]|^\[reopen-storm\]|test result|panicked at|^error(\[|:)' "$LOG" | head -40
exit $rc
