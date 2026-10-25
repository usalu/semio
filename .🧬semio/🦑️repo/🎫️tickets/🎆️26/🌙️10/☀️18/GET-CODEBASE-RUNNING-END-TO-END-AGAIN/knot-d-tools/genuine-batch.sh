#!/bin/sh
# usage: genuine-batch.sh <batch-name> <listfile>  (one repo-relative test file per line); one genuine birth per file
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
cd /Users/ueli/Documents/semio || exit 1
name="$1"; list="$2"
n=0
while IFS= read -r file; do
  [ -z "$file" ] && continue
  n=$((n+1))
  log="$C/🗑️generated/knot-d/$name-$n.log"
  s=$(date +%s)
  SEMIO_LANGUAGE=en nice -n 10 bun "$C/knot-d-tools/genuine.ts" "/Users/ueli/Documents/semio/$file" > "$log" 2>&1
  rc=$?
  pass=$(/usr/bin/grep -E "^ *[0-9]+ pass" "$log" | tail -1 | tr -s ' ')
  fail=$(/usr/bin/grep -E "^ *[0-9]+ fail" "$log" | tail -1 | tr -s ' ')
  printf '%s\trc=%s\t%ss\t%s\t%s\t%s\n' "$name-$n" "$rc" "$(( $(date +%s)-s ))" "$pass" "$fail" "$file" >> "$C/🗑️generated/knot-d/batch-$name.tsv"
done < "$list"
echo done >> "$C/🗑️generated/knot-d/batch-$name.tsv"
