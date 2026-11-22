#!/bin/sh
# usage: rework-market-parasol-render.sh "<r3-sheet args>" ...   one sheet per argument string, copied to 🗑️generated/rework-market-parasol/
cd "$(dirname "$0")" || exit 1
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
OUT="🗑️generated/rework-market-parasol"
mkdir -p "$OUT"
for x in "$@"; do
  # shellcheck disable=SC2086
  f=$("$BUN" r3-sheet.ts objects/market-parasol $x 2>&1 | grep -E "\.png|NOT DRAWN|rror" | tail -1)
  case "$f" in *.png) cp "$f" "$OUT/" && echo "$OUT/$(basename "$f")" ;; *) echo "$f" ;; esac
done
