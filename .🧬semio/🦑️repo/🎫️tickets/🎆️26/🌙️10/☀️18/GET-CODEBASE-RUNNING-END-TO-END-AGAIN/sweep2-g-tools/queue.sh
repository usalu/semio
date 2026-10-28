#!/bin/bash
# sweep2-g oracle queue: one target at a time (nice). usage: queue.sh <tag-prefix> <id>...  ; key list from 🗑️generated/sweep2-g/dump (or $LISTDIR).
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"; G="$C/🗑️generated/sweep2-g"
prefix=$1; shift
for id in "$@"; do
  name=$(printf '%s' "$id" | sed 's|[/@]|_|g; s|:|__|'); tag="$prefix-$name"
  list="${LISTDIR:-$G/dump}/$name.files"
  echo "$(date +%H:%M:%S) start $tag" >> "$G/queue.log"
  nice -n 10 bun "$C/sweep2-g-tools/target-run.mjs" "$tag" "$G/closure.jsonl" "$id" "$list" --rounds "${ROUNDS:-14}" --timeout 900 --extra "${EXTRA:-$G/c1-all.files}" > "$G/$tag.out" 2> "$G/$tag.err"
  echo "$(date +%H:%M:%S) end $tag rc=$?" >> "$G/queue.log"
done
echo "$(date +%H:%M:%S) queue done" >> "$G/queue.log"
