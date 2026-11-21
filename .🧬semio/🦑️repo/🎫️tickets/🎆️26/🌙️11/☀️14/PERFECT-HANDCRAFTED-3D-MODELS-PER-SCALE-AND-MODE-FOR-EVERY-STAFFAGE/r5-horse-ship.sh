#!/bin/sh
# Round 5 horse: model, details, 2D redraw, candidate validation, then (with --apply) root members, splice, validation.
set -e
cd "$(dirname "$0")"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
A="/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🐕️animals/horse/🔣️.json"
G="🗑️generated/r5-horse"
python3 r5-horse-model.py
python3 r5-horse-details.py
$BUN r5-horse-draw2d.ts | tail -4
python3 r5-candidate.py animals/horse "$A" "$G/members.json" "$G/🔣️.json"
$BUN r3-validate.ts --file "$G/🔣️.json" | tail -6
if [ "$1" = "--apply" ]; then
  python3 r3-horse-root.py members "$G/members.json"
  python3 volumes-insert.py volumes/animals/horse.json "$A"
  $BUN r3-validate.ts animals/horse --volumes | tail -6
fi
