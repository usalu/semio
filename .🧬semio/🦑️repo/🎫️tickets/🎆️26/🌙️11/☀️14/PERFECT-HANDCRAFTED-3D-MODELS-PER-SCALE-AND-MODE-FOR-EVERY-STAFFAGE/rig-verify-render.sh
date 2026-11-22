#!/bin/sh
# Renders every pose and two instants (0.3 and 0.7 of the duration) of every clip of the given assets with r3-sheet.ts and r3-2d.ts at 1:50, one process at a time: rig-verify-render.sh <category>/<slug> [...]; log in 🗑️generated/rig-verify/render.log
T="$(cd "$(dirname "$0")" && pwd)"
BUN=$(ls "$HOME"/Library/Caches/semio/runtimes/bun/*/bun | head -1)
C=/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog
mkdir -p "$T/🗑️generated/rig-verify"
cd "$T" || exit 1
for a in "$@"; do
  f=$(ls "$C"/*/"${a#*/}"/🔣️.json)
  python3 -c "
import json,sys
a=json.load(open(sys.argv[1]))['asset']
for p in a['poses']: print('--pose', p['id'])
for c in a['clips']:
    for f in (0.3,0.7): print('--clip', c['id'], '--t', round(f*c['durationS'],3))
" "$f" | while read -r args; do
    nice -n 10 "$BUN" r3-sheet.ts "$a" --scale 50 $args 2>&1 | tail -1
    nice -n 10 "$BUN" r3-2d.ts "$a" --scale 50 $args 2>&1 | tail -1
  done
done > "$T/🗑️generated/rig-verify/render.log" 2>&1
