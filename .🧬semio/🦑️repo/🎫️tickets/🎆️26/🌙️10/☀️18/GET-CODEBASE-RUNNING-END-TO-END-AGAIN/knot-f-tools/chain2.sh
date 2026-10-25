#!/bin/sh
# usage: chain2.sh <wait-pid> — after <wait-pid> ends: lint n2 (nx), then the official test targets sequentially (semio tool, no dependencies)
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
cd /Users/ueli/Documents/semio || exit 1
out="$C/🗑️generated/knot-f"
while kill -0 "$1" 2>/dev/null; do sleep 5; done
sh "$C/knot-f-tools/lint-dir.sh" n2 nx
unset SEMIO_SCRIPT_STALL_MS SEMIO_TEST_ARTIFACT_DIR
for t in @semio-tech/repo-lib:test-dependency-direction @semio-tech/repo-lib:test-package-body-policy @semio-tech/repo-test:test-discovery-boundaries @semio-tech/filesystem-observation:test-typescript @semio-tech/framework-process:test-routing-owners; do
 n=$(printf '%s' "$t" | tr '/@:' '___'); s=$(date +%s)
 nice -n 10 bun repo --tool=semio run "$t" --exclude-dependencies > "$out/o-$n.log" 2>&1
 echo "$t rc=$? $(( $(date +%s)-s ))s" >> "$out/verdicts.txt"
done
echo done > "$out/chain2.done"
