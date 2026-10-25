#!/bin/sh
# usage: jrun.sh <tag> <binary> <test-exe> [filters...]  runs the journeys executable directly (sequential), log in generated/dash3-d/<tag>.log
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
O="$C/🗑️generated/dash3-d"; tag=$1; bin=$2; exe=$3; shift 3
A=/tmp/sa3/$tag; [ -n "$DEEP" ] && A="$A/$(printf 'deep%.0s' $(seq 1 18))"; rm -rf /tmp/sa3/$tag; mkdir -p "$A"
SEMIO_TEST_CLI="$bin" SEMIO_TEST_ARTIFACT_DIR="$A" RUST_BACKTRACE=0 nice -n 10 "$exe" --test-threads=1 --nocapture "$@" > "$O/$tag.log" 2>&1
echo "rc=$?" > "$O/$tag.done"
