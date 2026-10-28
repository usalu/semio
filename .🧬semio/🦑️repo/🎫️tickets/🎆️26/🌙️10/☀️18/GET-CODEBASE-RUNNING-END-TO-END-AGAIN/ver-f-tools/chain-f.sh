#!/bin/sh
# usage: chain-f.sh — waits for chain-e, then test-native-storage and test-testing as separate official runs (one failing target must not hide the other)
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
cd /Users/ueli/Documents/semio || exit 1
out="$C/🗑️generated/ver-f"
while [ ! -f "$out/chain-e.done" ]; do sleep 20; done
unset SEMIO_SCRIPT_STALL_MS SEMIO_TEST_ARTIFACT_DIR
for t in test-native-storage test-testing; do
 s=$(date +%s); nice -n 10 bun repo --tool=bazel run @semio-tech/framework-process:$t > "$out/f-$t.log" 2>&1
 printf '%s\trc=%s\t%ss\t%s\n' "$t" "$?" "$(( $(date +%s)-s ))" "$(date +%T)" >> "$out/verdicts-f.tsv"
done
echo done > "$out/chain-f.done"
