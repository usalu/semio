#!/bin/sh
# usage: chain.sh <tag> <id>... runs the matching rows of batches.txt sequentially via official Bazel run-many (--parallel=1); summary in 🗑️generated/crate-f/chain.tsv
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
O="$C/🗑️generated/crate-f"
unset SEMIO_SCRIPT_STALL_MS
export SEMIO_LANGUAGE=en
tag=$1; shift
for want in "$@"; do
  while IFS='|' read -r id projects targets; do
    [ "$id" = "$want" ] || continue
    st=$(date +%H:%M:%S)
    nice -n 10 bun repo --tool=bazel run-many --projects="$projects" --targets="$targets" --parallel=1 > "$O/$tag-$id.log" 2>&1
    rc=$?
    printf '%s\t%s\trc=%s\t%s-%s\n' "$tag" "$id" "$rc" "$st" "$(date +%H:%M:%S)" >> "$O/chain.tsv"
  done < "$C/crate-f-tools/batches.txt"
done
echo "done $tag $(date +%H:%M:%S)" >> "$O/chain.tsv"
