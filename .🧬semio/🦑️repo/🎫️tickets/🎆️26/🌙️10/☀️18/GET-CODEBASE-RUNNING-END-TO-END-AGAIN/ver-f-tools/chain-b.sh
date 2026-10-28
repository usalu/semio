#!/bin/sh
# usage: chain-b.sh — knot-f's pending official targets one at a time (semio tool, no dependencies), then lint n3 (nx); verdicts in 🗑️generated/ver-f/verdicts-b.tsv
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
cd /Users/ueli/Documents/semio || exit 1
out="$C/🗑️generated/ver-f"
unset SEMIO_SCRIPT_STALL_MS SEMIO_TEST_ARTIFACT_DIR
for t in @semio-tech/repo-lib:test-package-body-policy @semio-tech/repo-test:test-discovery-boundaries @semio-tech/filesystem-observation:test-typescript @semio-tech/framework-process:test-routing-owners @semio-tech/repo-lib:test-dependency-direction; do
 n=$(printf '%s' "$t" | tr '/@:' '___'); s=$(date +%s)
 nice -n 10 bun repo --tool=semio run "$t" --exclude-dependencies > "$out/b-$n.log" 2>&1
 printf '%s\trc=%s\t%ss\t%s\n' "$t" "$?" "$(( $(date +%s)-s ))" "$(date +%T)" >> "$out/verdicts-b.tsv"
done
sh "$C/knot-f-tools/lint-dir.sh" n3 nx
cp "$C/🗑️generated/knot-f/verdicts.txt" "$out/verdicts-b-lint.txt" 2>/dev/null
echo done > "$out/chain-b.done"
