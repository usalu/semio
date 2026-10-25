#!/bin/sh
C="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
cd /Users/ueli/Documents/semio || exit 1
out="$C/🗑️generated/expc-d"
s=$(date +%s); SEMIO_LANGUAGE=en env -u SEMIO_SCRIPT_STALL_MS nice -n 10 bun repo --tool=bazel run @semio-tech/print:test > "$out/proof-p3-print-test-3.log" 2>&1; rc=$?
printf 'print-test-3\t%s\t%ss\tbun repo --tool=bazel run @semio-tech/print:test\n' "$rc" "$(( $(date +%s)-s ))" >> "$out/proofs.tsv"
echo done > "$out/proofs-p3.done"
