#!/bin/sh
# usage: chain-i.sh — waits for chain-h, then repo:test-runtime-bootstrap once more (node-only image law)
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
cd /Users/ueli/Documents/semio || exit 1
out="$C/🗑️generated/ver-f"
while [ ! -f "$out/chain-h.done" ]; do sleep 20; done
unset SEMIO_SCRIPT_STALL_MS SEMIO_TEST_ARTIFACT_DIR
s=$(date +%s); nice -n 10 bun repo --tool=bazel run repo:test-runtime-bootstrap > "$out/i-runtimeboot.log" 2>&1
printf 'runtimeboot3\trc=%s\t%ss\t%s\n' "$?" "$(( $(date +%s)-s ))" "$(date +%T)" >> "$out/verdicts-i.tsv"
echo done > "$out/chain-i.done"
