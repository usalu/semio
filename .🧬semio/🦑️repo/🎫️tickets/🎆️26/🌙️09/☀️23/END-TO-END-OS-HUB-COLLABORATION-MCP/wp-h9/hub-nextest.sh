#!/bin/zsh
# Hub level gate as the repo verb runs it (nextest, level profile, higher levels skipped), on the H9 private target.
# usage: hub-nextest.sh <quick|long> [cargo feature args...]
cd /Users/ueli/Documents/semio
LEVEL=$1; shift
export CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.tmp-ticket/wp-h9/target RUST_MIN_STACK=268435456 SEMIO_TEST_LEVEL=$LEVEL
export SEMIO_TEST_ARTIFACT_DIR=/Users/ueli/Documents/semio/.tmp-ticket/wp-h9/generated/test-artifacts-$LEVEL
mkdir -p "$SEMIO_TEST_ARTIFACT_DIR" && chmod 700 "$SEMIO_TEST_ARTIFACT_DIR"
case $LEVEL in quick) SKIP=(--skip long:: --skip exhaustive::) ;; long) SKIP=(--skip exhaustive::) ;; esac
echo "=== start $(date +%T) level=$LEVEL args: $*"
cargo nextest run -p semio-hub --profile $LEVEL --no-fail-fast "$@" -- $SKIP 2>&1 | /usr/bin/grep -E "Summary|FAIL|TIMEOUT|SIGSEGV|SIGABRT|LEAK|^error|Starting|PASS \[ *[0-9]{2,}" | tail -60
echo "EXIT ${pipestatus[1]}"
echo "=== done $(date +%T)"
