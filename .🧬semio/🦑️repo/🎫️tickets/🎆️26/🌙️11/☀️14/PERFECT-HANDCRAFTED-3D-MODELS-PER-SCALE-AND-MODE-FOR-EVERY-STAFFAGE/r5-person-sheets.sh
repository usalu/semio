#!/bin/sh
# 🖼️ Renders every sheet the round 5 brief asks to open for one human asset: body, structure and cut thermal at every scale, every pose at 1:50, one frame of every clip at 1:50, and the 2D drawings of every scale. Usage: sh r5-person-sheets.sh humans/<slug>
cd "$(dirname "$0")" || exit 1
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
A=$1
for s in 50 100 200 500; do
  "$BUN" r3-sheet.ts "$A" --scale $s --no-rig | tail -1
  "$BUN" r3-sheet.ts "$A" --scale $s --list structure | tail -1
  "$BUN" r3-sheet.ts "$A" --scale $s --list thermal --cut | tail -1
  "$BUN" r3-2d.ts "$A" --scale $s | tail -1
done
for p in $(python3 -c "import json,glob;a=json.load(open(glob.glob('/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/*/${A#*/}/🔣️.json')[0]))['asset'];print(' '.join(q['id'] for q in a['poses'] if not q.get('key')))"); do "$BUN" r3-sheet.ts "$A" --scale 50 --pose $p --no-rig | tail -1; done
for c in $(python3 -c "import json,glob;a=json.load(open(glob.glob('/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/*/${A#*/}/🔣️.json')[0]))['asset'];print(' '.join(q['id']+':'+str(round(q.get('durationS',1)*0.4,2)) for q in a['clips']))"); do "$BUN" r3-sheet.ts "$A" --scale 50 --clip ${c%%:*} --t ${c##*:} --no-rig | tail -1; done
