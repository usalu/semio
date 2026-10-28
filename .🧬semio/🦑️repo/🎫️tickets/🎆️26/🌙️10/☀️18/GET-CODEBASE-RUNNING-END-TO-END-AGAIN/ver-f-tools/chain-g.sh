#!/bin/sh
# usage: chain-g.sh — waits for chain-f, then repo:test-runtime-bootstrap again (devcontainer storage requirement fixed)
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
cd /Users/ueli/Documents/semio || exit 1
out="$C/🗑️generated/ver-f"
while [ ! -f "$out/chain-f.done" ]; do sleep 20; done
unset SEMIO_SCRIPT_STALL_MS SEMIO_TEST_ARTIFACT_DIR
s=$(date +%s); nice -n 10 bun repo --tool=bazel run repo:test-runtime-bootstrap > "$out/g-runtimeboot.log" 2>&1
printf 'runtimeboot2\trc=%s\t%ss\t%s\n' "$?" "$(( $(date +%s)-s ))" "$(date +%T)" >> "$out/verdicts-g.tsv"
echo done > "$out/chain-g.done"
