#!/bin/sh
# usage: chain-e.sh — post-freeze official reruns on the patched tree, one at a time; verdicts in 🗑️generated/ver-f/verdicts-e.tsv
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
cd /Users/ueli/Documents/semio || exit 1
out="$C/🗑️generated/ver-f"
unset SEMIO_SCRIPT_STALL_MS SEMIO_TEST_ARTIFACT_DIR
run(){ tag="$1"; shift; s=$(date +%s); nice -n 10 bun repo "$@" > "$out/e-$tag.log" 2>&1; printf '%s\trc=%s\t%ss\t%s\n' "$tag" "$?" "$(( $(date +%s)-s ))" "$(date +%T)" >> "$out/verdicts-e.tsv"; }
run a2 --tool=bazel run-many --projects=@semio-tech/framework-process --targets=test-testing,test-native-storage --parallel=1 --exclude-dependencies
run runtimeboot --tool=bazel run repo:test-runtime-bootstrap
run uitest --tool=bazel run @semio-tech/ui-react:test -- react-package-export
for c in "$out"/cache-rest-chunk.aag "$out"/cache-rest-chunk.aah "$out"/cache-rest-chunk.aai "$out"/cache-rest-chunk.aaj "$out"/cache-rest-chunk.aak "$out"/cache-rest-chunk.aal "$out"/cache-rest-chunk.aam; do
 [ -f "$c" ] || continue
 b=$(basename "$c"); run "$b" --tool=bazel run-many --projects=repo --targets="$(paste -sd, "$c")" --parallel=1 --exclude-dependencies
done
for c in "$out"/cache-red-chunk.aa?; do
 case "$c" in *.log) continue;; esac
 [ -f "$c" ] || continue
 b=$(basename "$c"); run "$b" --tool=bazel run-many --projects=repo --targets="$(paste -sd, "$c")" --parallel=1 --exclude-dependencies
done
run pixels --tool=bazel run semio-framework-pixels:test
echo done > "$out/chain-e.done"
