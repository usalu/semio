#!/bin/sh
# 🖼️ Round 5, vegetation (agave, bamboo-clump, hosta-clump, miscanthus-grass): renders every sheet the brief asks for, one after another: body, structure and cut thermal at every authored scale, every pose at 1:50, one frame of every clip, and the product 2D of every drawn scale. Usage: sh r5-vegetation-sheets.sh <slug> [scales]
cd "$(dirname "$0")"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
A="vegetation/$1"
SCALES=${2:-"50 100 200 500"}
for s in $SCALES; do
  "$BUN" r3-sheet.ts "$A" --scale "$s" --no-labels 2>&1 | tail -1
  "$BUN" r3-sheet.ts "$A" --scale "$s" --list structure --no-labels 2>&1 | tail -1
  "$BUN" r3-sheet.ts "$A" --scale "$s" --list thermal --cut --no-labels 2>&1 | tail -1
  "$BUN" r3-2d.ts "$A" --scale "$s" 2>&1 | tail -1
done
if [ -z "$3" ]; then
POSES=$(python3 -c "import json;d=json.load(open('/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🌳️vegetation/$1/🔣️.json'))['asset'];print(' '.join(p['id'] for p in d['poses'] if p['id']!=d['presentation']['defaultPose']))")
for p in $POSES; do "$BUN" r3-sheet.ts "$A" --scale 50 --pose "$p" --no-labels 2>&1 | tail -1; done
CLIPS=$(python3 -c "import json;d=json.load(open('/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🌳️vegetation/$1/🔣️.json'))['asset'];print(' '.join(c['id']+':'+str(round(c['durationS']*0.4,2)) for c in d['clips']))")
for c in $CLIPS; do "$BUN" r3-sheet.ts "$A" --scale 50 --clip "${c%%:*}" --t "${c##*:}" --no-labels 2>&1 | tail -1; done
fi
