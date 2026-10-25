#!/bin/sh
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
out="$C/🗑️generated/alloc-f"; mkdir -p "$out"
unset SEMIO_SCRIPT_STALL_MS SEMIO_TEST_ARTIFACT_DIR
tag="$1"; shift
for t in "$@"; do
  s=$(date +%s)
  nice -n 10 bun repo --tool=bazel run-many --projects=@semio-tech/framework-process --targets="$t" --parallel=1 --exclude-dependencies > "$out/$tag-$t.log" 2>&1
  rc=$?
  printf '%s\t%s\t%s\t%ss\n' "$t" "$rc" "$(date +%T)" "$(( $(date +%s) - s ))" >> "$out/$tag.tsv"
done
echo done > "$out/$tag.done"
