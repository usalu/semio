#!/bin/sh
# 🚗 Renders the round-4 review set of one vehicle asset, one image at a time: r4-g14-render.sh <category/slug> [scales]
cd "$(dirname "$0")" || exit 1
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
A="$1"
SCALES="${2:-50 100 200 500}"
for N in $SCALES; do
  "$BUN" r3-sheet.ts "$A" --scale "$N" 2>&1 | tail -2
  "$BUN" r3-sheet.ts "$A" --scale "$N" --list structure 2>&1 | tail -2
  "$BUN" r3-sheet.ts "$A" --scale "$N" --list thermal --cut 2>&1 | tail -2
  "$BUN" r3-2d.ts "$A" --scale "$N" 2>&1 | tail -2
done
