#!/bin/sh
C="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
sh "$C/expc-d-tools/lint-dir.sh" r4
cd /Users/ueli/Documents/semio || exit 1
out="$C/🗑️generated/expc-d"
run() { name="$1"; shift; s=$(date +%s); SEMIO_LANGUAGE=en env -u SEMIO_SCRIPT_STALL_MS nice -n 10 "$@" > "$out/proof-p2-$name.log" 2>&1; rc=$?; printf '%s\t%s\t%ss\t%s\n' "$name" "$rc" "$(( $(date +%s)-s ))" "$*" >> "$out/proofs.tsv"; }
run course-typecheck-2 bun repo --tool=bazel run @teaching/architecture-course:typecheck
run print-test-2 bun repo --tool=bazel run @semio-tech/print:test
echo done > "$out/proofs-p2.done"
