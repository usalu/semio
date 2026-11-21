#!/bin/sh
# 🖼️ Render every r5 grading sheet of one object asset: body, thermal (cut), structure at every scale, 2D at every scale, every pose at 1:50 and one frame per clip.
set -u
cd "$(dirname "$0")"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
A=$1; shift
WHAT=${1:-all}
if [ "$WHAT" = all ] || [ "$WHAT" = sheets ]; then
  for N in 50 100 200 500; do
    $BUN r3-sheet.ts "$A" --scale $N 2>&1 | tail -1
    $BUN r3-sheet.ts "$A" --scale $N --list thermal --cut 2>&1 | tail -1
    $BUN r3-sheet.ts "$A" --scale $N --list structure 2>&1 | tail -1
  done
fi
if [ "$WHAT" = all ] || [ "$WHAT" = 2d ]; then
  for N in 50 100 200 500; do $BUN r3-2d.ts "$A" --scale $N 2>&1 | tail -1; done
fi
if [ "$WHAT" = all ] || [ "$WHAT" = poses ]; then
  for P in $(python3 -c "import json,sys;a=json.load(open(sys.argv[1]))['asset'];print(' '.join(p['id'] for p in a['poses']))" "$(ls -d /Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/*/"${A#*/}"/🔣️.json)"); do
    $BUN r3-sheet.ts "$A" --scale 50 --pose $P 2>&1 | tail -1
  done
  python3 -c "import json,sys;a=json.load(open(sys.argv[1]))['asset'];[print(c['id'],round(c['durationS']*0.4,3)) for c in a['clips']]" "$(ls -d /Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/*/"${A#*/}"/🔣️.json)" | while read C T; do
    $BUN r3-sheet.ts "$A" --scale 50 --clip $C --t $T 2>&1 | tail -1
  done
fi
