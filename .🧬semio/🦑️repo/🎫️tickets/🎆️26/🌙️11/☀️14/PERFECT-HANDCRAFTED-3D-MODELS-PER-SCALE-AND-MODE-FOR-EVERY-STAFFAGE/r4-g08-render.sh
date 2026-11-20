#!/bin/sh
# Renders every review sheet of one asset, one after the other: r4-g08-render.sh <asset> [2d]
cd "$(dirname "$0")" || exit 1
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
A="$1"
for N in 50 100 200 500; do
  "$BUN" r3-sheet.ts "$A" --scale $N 2>&1 | tail -2
  "$BUN" r3-sheet.ts "$A" --scale $N --list structure 2>&1 | tail -2
  "$BUN" r3-sheet.ts "$A" --scale $N --list thermal --cut 2>&1 | tail -2
  if [ "$2" = "2d" ]; then "$BUN" r3-2d.ts "$A" --scale $N 2>&1 | tail -2; fi
done
