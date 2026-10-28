#!/bin/sh
# usage: chain-c.sh — items 6a/6b, 3, 4 (red suites then the rest, chunks of 8), 6c; one official run at a time; verdicts in 🗑️generated/ver-f/verdicts-c.tsv
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
cd /Users/ueli/Documents/semio || exit 1
out="$C/🗑️generated/ver-f"
unset SEMIO_SCRIPT_STALL_MS SEMIO_TEST_ARTIFACT_DIR
run(){ tag="$1"; shift; s=$(date +%s); nice -n 10 bun repo "$@" > "$out/c-$tag.log" 2>&1; printf '%s\trc=%s\t%ss\t%s\n' "$tag" "$?" "$(( $(date +%s)-s ))" "$(date +%T)" >> "$out/verdicts-c.tsv"; }
run uitest --tool=bazel run @semio-tech/ui-react:test -- "🧰️framework/🔨️modules/🖱️ui/🧪️tests/📦️react-package-export/🟦️.ts"
run compnative --tool=bazel run @semio-tech/framework-compiler-rs:test-native
run runtimeboot --tool=bazel run repo:test-runtime-bootstrap
for list in cache-red cache-rest; do
 rm -f "$out/$list-chunk."*
 split -l 8 -a 3 "$C/ver-f-tools/$list.txt" "$out/$list-chunk."
 for c in "$out/$list-chunk."*; do
  b=$(basename "$c"); targets=$(paste -sd, "$c")
  run "$b" --tool=bazel run-many --projects=repo --targets="$targets" --parallel=1 --exclude-dependencies
 done
done
run pixels --tool=bazel run semio-framework-pixels:test
echo done > "$out/chain-c.done"
