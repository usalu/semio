#!/bin/bash
# keys3-g sweep: sequential oracle runs (ONE at a time) of cached targets whose key lists exist in undeclared-g's lists-d; per target a report in undeclared-g's reports/ (tag k3-*) and a line in keys3-g's sweep.log.
# usage: sweep.sh <tag>|<id>|<list-basename> ...
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"; U="$C/undeclared-g-tools"; G="$C/🗑️generated/undeclared-g"; G3="$C/🗑️generated/keys3-g"
for spec in "$@"; do
  tag=${spec%%|*}; rest=${spec#*|}; id=${rest%%|*}; list=${rest#*|}
  echo "$(date +%H:%M:%S) start $tag $id" >> "$G3/sweep.log"
  nice -n 10 bun "$U/target-run.mjs" "$tag" "$G/closure-d.jsonl" "$id" "$G/lists-d/$list.files" --rounds 6 --timeout 900 > "$G3/sweep-$tag.out" 2> "$G3/sweep-$tag.err"
  echo "$(date +%H:%M:%S) end $tag rc=$?" >> "$G3/sweep.log"
done
echo "$(date +%H:%M:%S) queue done" >> "$G3/sweep.log"
