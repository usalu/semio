#!/bin/sh
# official-test.sh : the official `repo-lib:test-package-body-policy` target (graph policy current), measured
cd /Users/ueli/Documents/semio || exit 1
export SEMIO_BUILD_GRAPH_POLICY=current SEMIO_TEST_ARTIFACT_DIR="/Users/ueli/.rust-g-art/official"; mkdir -p "$SEMIO_TEST_ARTIFACT_DIR"
unset SEMIO_SCRIPT_STALL_MS
C="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
nice -n 10 python3 "$C/rust-g-tools/measure.py" official-test "$C/🗑️generated/rust-g/runs.jsonl" -- bun repo run @semio-tech/repo-lib:test-package-body-policy
echo "rc=$?"
