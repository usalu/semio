#!/bin/sh
cd /Users/ueli/Documents/semio || exit 1
D=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
list="$1"; out="$2"; start="$3"; end="$4"
i=0
while IFS= read -r f; do
  i=$((i+1)); [ "$i" -lt "$start" ] && continue; [ "$i" -gt "$end" ] && break
  log="$D/🗑️generated/dbg2-d/t/$i.log"; mkdir -p "$D/🗑️generated/dbg2-d/t"
  nice -n 10 perl -e 'alarm 240; exec @ARGV' bun test "./$f" > "$log" 2>&1; rc=$?
  sum=$(grep -E '^ *[0-9]+ (pass|fail)' "$log" | tr '\n' ' ')
  echo "$i rc=$rc $sum $(echo "$f" | awk -F/ '{print $(NF-2)"/"$(NF-1)"/"$NF}')" >> "$out"
done < "$list"
