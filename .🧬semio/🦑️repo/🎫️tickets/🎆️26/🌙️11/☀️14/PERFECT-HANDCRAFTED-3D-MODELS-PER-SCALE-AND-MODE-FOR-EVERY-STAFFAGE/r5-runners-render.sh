#!/bin/sh
# 🖼️ Renders the round 5 review sheets of one asset, one after the other: r5-runners-render.sh <category/slug> <scales...> [-- extra r3-sheet args]; per scale body, structure and thermal --cut.
cd "$(dirname "$0")" || exit 1
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
a="$1"; shift
for s in "$@"; do
  "$BUN" r3-sheet.ts "$a" --scale "$s" --no-labels 2>&1 | grep -v "^/" | grep . 
  "$BUN" r3-sheet.ts "$a" --scale "$s" --list structure --no-labels 2>&1 | grep -v "^/" | grep .
  "$BUN" r3-sheet.ts "$a" --scale "$s" --list thermal --cut --no-labels 2>&1 | grep -v "^/" | grep .
done
echo rendered
