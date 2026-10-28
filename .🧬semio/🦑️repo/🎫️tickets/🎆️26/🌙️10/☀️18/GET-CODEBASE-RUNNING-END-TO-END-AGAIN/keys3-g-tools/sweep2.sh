#!/bin/bash
# keys3-g sweep2: waits for the first queue to end (one oracle run at a time on this shared machine), then runs the listed target ids one by one against closure-k3.jsonl; the key list of each target is taken from keys3-g's dump (current generation) or undeclared-g's lists.
# usage: sweep2.sh <tag-prefix> <id>...
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"; U="$C/undeclared-g-tools"; G="$C/🗑️generated/undeclared-g"; G3="$C/🗑️generated/keys3-g"
prefix=$1; shift
for _ in $(seq 1 360); do /usr/bin/grep -q "queue done" "$G3/sweep.log" 2>/dev/null && break; sleep 20; done
n=0
for id in "$@"; do
  n=$((n+1)); name=$(printf '%s' "$id" | sed 's|[/@]|_|g; s|:|__|'); list=""
  for dir in "$G3/dump" "$G/lists-a" "$G/lists-d" "$G/lists-c" "$G/lists"; do [ -f "$dir/$name.files" ] && { list="$dir/$name.files"; break; }; done
  tag="$prefix$n"
  [ -z "$list" ] && { echo "$(date +%H:%M:%S) skip $tag $id (no key list)" >> "$G3/sweep.log"; continue; }
  echo "$(date +%H:%M:%S) start $tag $id" >> "$G3/sweep.log"
  nice -n 10 bun "$U/target-run.mjs" "$tag" "$G3/closure-k3.jsonl" "$id" "$list" --rounds 12 --timeout 900 --extra "$G3/c1-all.files" > "$G3/sweep-$tag.out" 2> "$G3/sweep-$tag.err"
  echo "$(date +%H:%M:%S) end $tag $id rc=$?" >> "$G3/sweep.log"
done
echo "$(date +%H:%M:%S) queue2 done" >> "$G3/sweep.log"
