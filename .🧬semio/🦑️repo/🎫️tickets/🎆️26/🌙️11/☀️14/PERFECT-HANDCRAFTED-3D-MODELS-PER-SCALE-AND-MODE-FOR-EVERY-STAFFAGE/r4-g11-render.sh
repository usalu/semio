#!/bin/sh
# 🖼️ Round 4 group G11: renders the review set of one asset (body, structure, thermal cut at every scale, one pose and one clip frame at 1:50, the 2D drawings) one after the other. Usage: ./r4-g11-render.sh <slug> <pose> <clip> <t>
cd "$(dirname "$0")"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
for n in 50 100 200 500; do
  "$BUN" r3-sheet.ts "humans/$1" --scale $n --no-rig 2>&1 | grep -i "not drawn"
  "$BUN" r3-sheet.ts "humans/$1" --scale $n --list structure --no-rig 2>&1 | grep -i "not drawn"
  "$BUN" r3-sheet.ts "humans/$1" --scale $n --list thermal --cut --no-rig 2>&1 | grep -i "not drawn"
  "$BUN" r3-2d.ts "humans/$1" --scale $n 2>&1 | tail -1
done
"$BUN" r3-sheet.ts "humans/$1" --scale 50 --pose "$2" --no-rig 2>&1 | tail -1
"$BUN" r3-sheet.ts "humans/$1" --scale 50 --clip "$3" --t "$4" --no-rig 2>&1 | tail -1
"$BUN" r3-sheet.ts "humans/$1" --scale 50 --list structure --pose "$2" --no-rig 2>&1 | tail -1
