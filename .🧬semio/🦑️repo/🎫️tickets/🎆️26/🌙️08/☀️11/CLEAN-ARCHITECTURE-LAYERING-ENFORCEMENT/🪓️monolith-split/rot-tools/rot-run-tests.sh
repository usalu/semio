#!/bin/sh
cd /Users/ueli/Documents/semio || exit 2
F=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
LIST="$1"
TAG="$2"
OUT="$F/🗑️generated/rot-$TAG.tsv"
LOG="$F/🗑️generated/rot-$TAG.log"
: > "$OUT"; : > "$LOG"
while IFS= read -r file; do
  [ -z "$file" ] && continue
  printf '\n=== %s\n' "$file" >> "$LOG"
  res=$(perl -e 'alarm 900; exec @ARGV' -- bun test "./$file" 2>&1 | tee -a "$LOG" | /usr/bin/grep -E '^ *[0-9]+ (pass|fail)$|error: Cannot find|Ran [0-9]+ tests' | tr '\n' ' ')
  printf '%s\t%s\n' "$file" "$res" >> "$OUT"
done < "$LIST"
echo done > "$F/🗑️generated/rot-$TAG.status"
