#!/bin/sh
# usage: batch.sh <tag> <tool> <listfile>  -> official `repo:<target>` targets sequentially; resumable; summary TSV <out>/<tag>.tsv
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
out="$C/🗑️generated/cache-f"; mkdir -p "$out"
unset SEMIO_SCRIPT_STALL_MS SEMIO_TEST_ARTIFACT_DIR
tag="$1"; tool="$2"; list="$3"
while read -r t; do
  [ -z "$t" ] && continue
  [ -f "$out/$tag-$t.rc" ] && continue
  s=$(date +%s)
  nice -n 10 bun repo --tool="$tool" run "repo:$t" --exclude-dependencies > "$out/$tag-$t.log" 2>&1
  rc=$?
  echo $rc > "$out/$tag-$t.rc"
  printf '%s\t%s\t%s\t%ss\n' "$t" "$rc" "$(date +%T)" "$(( $(date +%s) - s ))" >> "$out/$tag.tsv"
done < "$list"
echo done > "$out/$tag.done"
