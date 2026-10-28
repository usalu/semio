#!/bin/sh
# usage: chain-h.sh — waits for chain-g, then test-testing once more (file-observation control test follows the Bazel action's unbounded envelope)
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
cd /Users/ueli/Documents/semio || exit 1
out="$C/🗑️generated/ver-f"
while [ ! -f "$out/chain-g.done" ]; do sleep 20; done
unset SEMIO_SCRIPT_STALL_MS SEMIO_TEST_ARTIFACT_DIR
s=$(date +%s); nice -n 10 bun repo --tool=bazel run @semio-tech/framework-process:test-testing > "$out/h-test-testing.log" 2>&1
printf 'test-testing2\trc=%s\t%ss\t%s\n' "$?" "$(( $(date +%s)-s ))" "$(date +%T)" >> "$out/verdicts-h.tsv"
echo done > "$out/chain-h.done"
