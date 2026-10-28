#!/bin/sh
# usage: chain-d.sh — waits for chain-c, then item 6a with the vitest filter that matches (substring of the test path)
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
cd /Users/ueli/Documents/semio || exit 1
out="$C/🗑️generated/ver-f"
while [ ! -f "$out/chain-c.done" ]; do sleep 20; done
unset SEMIO_SCRIPT_STALL_MS SEMIO_TEST_ARTIFACT_DIR
s=$(date +%s); nice -n 10 bun repo --tool=bazel run @semio-tech/ui-react:test -- react-package-export > "$out/c-uitest2.log" 2>&1
printf 'uitest2\trc=%s\t%ss\t%s\n' "$?" "$(( $(date +%s)-s ))" "$(date +%T)" >> "$out/verdicts-c.tsv"
echo done > "$out/chain-d.done"
