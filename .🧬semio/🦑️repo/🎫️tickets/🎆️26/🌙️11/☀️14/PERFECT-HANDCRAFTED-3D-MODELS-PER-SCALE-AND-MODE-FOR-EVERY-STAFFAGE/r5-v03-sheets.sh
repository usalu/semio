#!/bin/sh
# 🚲 Round 5 V03: renders every sheet the brief asks to open for one two-wheeler (body, structure, thermal cut at every scale, every pose at 1:50, one frame of every clip at 1:50, 2D of every scale) into 🗑️generated/r5-<slug>/<tag>/. Usage: sh r5-v03-sheets.sh vehicles/<slug> <tag> [scales]
cd "$(dirname "$0")" || exit 1
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
A="$1"; SLUG="${A#*/}"; OUT="🗑️generated/r5-$SLUG/$2"; SC="${3:-50 100 200 500}"; mkdir -p "$OUT"
F="/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🚜️vehicles/$SLUG/🔣️.json"
rm -f 🗑️generated/r3-sheet/vehicles/$SLUG.* 🗑️generated/r3-2d/vehicles/$SLUG.*
for N in $SC; do
  for L in "--list body" "--list structure" "--list thermal --cut"; do $BUN r3-sheet.ts "$A" --scale $N $L 2>&1 | grep -i "not drawn"; done
  $BUN r3-2d.ts "$A" --scale $N > /dev/null 2>&1 || echo "2D $N failed"
done
if [ -z "$3" ]; then
  for P in $(python3 -c "import json;a=json.load(open('$F'))['asset'];print(' '.join(q['id'] for q in a['poses']))"); do $BUN r3-sheet.ts "$A" --scale 50 --pose $P --no-rig 2>&1 | grep -i "not drawn"; done
  for C in $(python3 -c "import json;a=json.load(open('$F'))['asset'];print(' '.join(q['id']+':'+str(round(q.get('durationS',1)*0.45,2)) for q in a['clips']))"); do $BUN r3-sheet.ts "$A" --scale 50 --clip ${C%%:*} --t ${C##*:} --no-rig 2>&1 | grep -i "not drawn"; done
fi
cp 🗑️generated/r3-sheet/vehicles/$SLUG.* "$OUT"/ 2>/dev/null; cp 🗑️generated/r3-2d/vehicles/$SLUG.* "$OUT"/ 2>/dev/null
ls "$OUT" | wc -l
