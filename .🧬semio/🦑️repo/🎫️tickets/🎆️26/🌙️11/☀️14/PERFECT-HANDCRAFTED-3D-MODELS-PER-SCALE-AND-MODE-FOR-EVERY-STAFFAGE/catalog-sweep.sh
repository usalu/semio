#!/bin/sh
# 🧹 Validate every catalog asset as published (schema + validateAsset, no measured rules), one process at a time.
cd "$(dirname "$0")"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
OUT=🗑️generated/sweep/$(date +%H%M).tsv
for p in /Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/*/*/🔣️.json; do
  slug=$(basename "$(dirname "$p")"); cat=$(basename "$(dirname "$(dirname "$p")")" | LC_ALL=C sed 's/[^a-z-]//g')
  last=$(nice -n 10 "$BUN" r3-validate.ts "$cat/$slug" --catalog-only 2>&1 | tail -1 | cut -c1-200)
  printf '%s/%s\t%s\n' "$cat" "$slug" "$last" >> "$OUT"
done
echo "$OUT"
