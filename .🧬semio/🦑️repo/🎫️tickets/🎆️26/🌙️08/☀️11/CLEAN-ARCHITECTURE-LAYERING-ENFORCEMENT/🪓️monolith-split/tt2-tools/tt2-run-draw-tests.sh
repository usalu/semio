#!/bin/sh
cd /Users/ueli/Documents/semio || exit 2
F=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
TAG="$1"
OUT="$F/🗑️generated/tt2-draw-tests-$TAG.tsv"
LOG="$F/🗑️generated/tt2-draw-tests-$TAG.log"
: > "$OUT"; : > "$LOG"
while IFS= read -r file; do
  [ -z "$file" ] && continue
  printf '\n=== %s\n' "$file" >> "$LOG"
  res=$(perl -e 'alarm 900; exec @ARGV' -- bun test "./$file" 2>&1 | tee -a "$LOG" | /usr/bin/grep -E '^ *[0-9]+ (pass|fail)$|error: Cannot find|Ran [0-9]+ tests' | tr '\n' ' ')
  printf '%s\t%s\n' "$file" "$res" >> "$OUT"
done < "$F/🗑️generated/tt2-draw-test-files.txt"
echo done > "$F/🗑️generated/tt2-draw-tests-$TAG.status"
