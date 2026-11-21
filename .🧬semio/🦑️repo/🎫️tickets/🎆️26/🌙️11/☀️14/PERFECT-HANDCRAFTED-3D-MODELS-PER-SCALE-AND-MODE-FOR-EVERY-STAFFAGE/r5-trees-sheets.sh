#!/bin/sh
# 🖼️ Renders the r3 sheets of one tree into 🗑️generated/r5-<slug>/<tag>/: body at every scale, structure and cut thermal at every scale, every pose at 1:50, one clip frame, and the 2D at every scale. Usage: r5-trees-sheets.sh <slug> <tag> [quick]
set -e
cd "$(dirname "$0")"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
A="vegetation/$1"; O="🗑️generated/r5-$1/$2"; mkdir -p "$O"
G="🗑️generated/r3-sheet/vegetation"
for s in 50 100 200 500; do
  "$BUN" r3-sheet.ts "$A" --scale $s --no-labels >/dev/null
  [ "$3" = quick ] && continue
  "$BUN" r3-sheet.ts "$A" --scale $s --list structure --no-labels >/dev/null
  "$BUN" r3-sheet.ts "$A" --scale $s --list thermal --cut --no-labels >/dev/null
done
if [ "$3" != quick ]; then
  for p in $(python3 -c "import json,sys,glob;f=glob.glob('../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🌳️vegetation/$1/🔣️.json')[0];a=json.load(open(f))['asset'];print(' '.join(x['id'] for x in a['poses'] if x['id']!=a['presentation']['defaultPose']))"); do
    "$BUN" r3-sheet.ts "$A" --scale 50 --pose $p --no-labels >/dev/null
  done
  c=$(python3 -c "import json,glob;f=glob.glob('../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🌳️vegetation/$1/🔣️.json')[0];a=json.load(open(f))['asset'];print(a['clips'][1]['id'])")
  "$BUN" r3-sheet.ts "$A" --scale 50 --clip $c --t 0.6 --no-labels >/dev/null
  for s in 50 100 200 500; do "$BUN" r3-2d.ts "$A" --scale $s >/dev/null 2>&1 || echo "2d $s failed"; done
  cp 🗑️generated/r3-2d/vegetation/$1.* "$O/"
fi
cp "$G"/$1.* "$O/"
ls "$O"
