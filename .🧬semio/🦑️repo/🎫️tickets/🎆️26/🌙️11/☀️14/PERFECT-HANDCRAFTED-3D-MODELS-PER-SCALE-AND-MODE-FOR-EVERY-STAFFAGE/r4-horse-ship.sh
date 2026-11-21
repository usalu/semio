#!/bin/sh
# Round 4 horse: the whole ship chain (model, detail strokes, 2D redraw, root members, splice, validation).
set -e
cd "$(dirname "$0")"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
A="/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🐕️animals/horse/🔣️.json"
python3 r3-horse-model.py
python3 r3-horse-details.py
$BUN r3-horse-draw2d.ts | tail -3
python3 r3-horse-root.py members 🗑️generated/r3-horse/members.json
python3 volumes-insert.py volumes/animals/horse.json "$A"
$BUN r3-validate.ts animals/horse --volumes | tail -4
$BUN volumes-check.ts animals/horse --product | tail -1
