#!/bin/sh
# 🪜 Round 5 (C03 components): renders body, structure, cut thermal at every scale, the 2D of every scale, every pose at 1:50 and one clip frame of one asset, and copies them to 🗑️generated/r5-<slug>/<tag>/. Usage: sh r5-c03-sheets.sh <category/slug> <tag> "<poses>" "<clip> <t>" [scales]
cd "$(dirname "$0")" || exit 1
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
A="$1"; TAG="$2"; POSES="$3"; CLIP="$4"; SCALES="${5:-50 100 200 500}"
SLUG=$(basename "$A"); CAT=$(dirname "$A")
OUT="🗑️generated/r5-$SLUG/$TAG"; mkdir -p "$OUT"; touch "$OUT/.start"; sleep 1
for N in $SCALES; do
  for L in "--list body" "--list structure" "--list thermal --cut"; do $BUN r3-sheet.ts "$A" --scale $N $L 2>&1 | grep -i "not drawn"; done
  $BUN r3-2d.ts "$A" --scale $N > /dev/null 2>&1 || echo "2d failed $N"
done
for P in $POSES; do $BUN r3-sheet.ts "$A" --scale 50 --pose $P 2>&1 | grep -i "not drawn"; done
set -- $CLIP
[ -n "$1" ] && $BUN r3-sheet.ts "$A" --scale 50 --clip $1 --t $2 2>&1 | grep -i "not drawn"
find 🗑️generated/r3-sheet/$CAT 🗑️generated/r3-2d/$CAT -name "$SLUG.s*.png" -newer "$OUT/.start" -exec cp {} "$OUT"/ \;
ls "$OUT"
echo done "$A"
