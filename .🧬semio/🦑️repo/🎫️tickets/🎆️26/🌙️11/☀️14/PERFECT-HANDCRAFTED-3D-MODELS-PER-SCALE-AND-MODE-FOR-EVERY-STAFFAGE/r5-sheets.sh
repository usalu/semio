#!/bin/sh
# 🖼️ Round 5: renders every sheet the brief asks for (body, structure, thermal cut at every scale; every pose at 1:50; one clip frame; the 2D at every scale) for one asset and copies them to 🗑️generated/r5-<slug>/<tag>/. Usage: sh r5-sheets.sh <category/slug> <tag> "<pose ids>" "<clip id> <t>"
cd "$(dirname "$0")" || exit 1
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
A="$1"; SLUG="${A#*/}"; OUT="🗑️generated/r5-$SLUG/$2"; mkdir -p "$OUT"
for N in 50 100 200 500; do
  for L in "--list body" "--list structure" "--list thermal --cut"; do $BUN r3-sheet.ts "$A" --scale $N $L 2>&1 | grep -i "not drawn"; done
  $BUN r3-2d.ts "$A" --scale $N > /dev/null 2>&1 || echo "2D $N failed"
done
for P in $3; do $BUN r3-sheet.ts "$A" --scale 50 --pose $P 2>&1 | grep -i "not drawn"; done
set -- $4
[ -n "$1" ] && $BUN r3-sheet.ts "$A" --scale 50 --clip $1 --t $2 2>&1 | grep -i "not drawn"
cp 🗑️generated/r3-sheet/${A%/*}/$SLUG.* "$OUT"/ ; cp 🗑️generated/r3-2d/${A%/*}/$SLUG.* "$OUT"/
ls "$OUT" | wc -l
