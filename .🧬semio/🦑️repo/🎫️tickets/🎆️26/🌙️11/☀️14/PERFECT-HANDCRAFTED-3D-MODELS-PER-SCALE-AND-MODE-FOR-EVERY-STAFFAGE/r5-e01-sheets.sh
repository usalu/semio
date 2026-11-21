#!/bin/sh
# 🖼️ Round 5 (E01 appliances): renders the after-set of one asset (body, structure, cut thermal at every scale, 2D of every drawn scale, every pose at 1:50, one clip frame) into the tools' own folders. Usage: sh r5-e01-sheets.sh <category/slug> "<poses>" "<clip> <t>"
cd "$(dirname "$0")" || exit 1
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
A="$1"
for N in 50 100 200 500; do
  for L in "--list body" "--list structure" "--list thermal --cut"; do $BUN r3-sheet.ts "$A" --scale $N $L 2>&1 | grep -i "not drawn"; done
  $BUN r3-2d.ts "$A" --scale $N > /dev/null 2>&1
done
for P in $2; do $BUN r3-sheet.ts "$A" --scale 50 --pose $P 2>&1 | grep -i "not drawn"; done
set -- $3
[ -n "$1" ] && $BUN r3-sheet.ts "$A" --scale 50 --clip $1 --t $2 2>&1 | grep -i "not drawn"
echo done "$A"
