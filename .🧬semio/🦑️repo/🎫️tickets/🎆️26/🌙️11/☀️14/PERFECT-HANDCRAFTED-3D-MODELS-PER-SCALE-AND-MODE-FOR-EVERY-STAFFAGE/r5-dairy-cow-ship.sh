#!/bin/sh
# Round 5 dairy cow: model, 2D redraw, candidate validation, then (with --apply) root members, splice, validation.
set -e
cd "$(dirname "$0")"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
A="/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🐕️animals/dairy-cow/🔣️.json"
G="🗑️generated/r5-dairy-cow"
python3 r5-dairy-cow-model.py
$BUN r5-dairy-cow-draw2d.ts | tail -6
python3 r5-candidate.py animals/dairy-cow "$A" "$G/members.json" "$G/🔣️.json"
$BUN r3-validate.ts --file "$G/🔣️.json" | tail -6
if [ "$1" = "--apply" ]; then
  python3 r3-dairy-cow-root.py "$G/members.json"
  python3 volumes-insert.py volumes/animals/dairy-cow.json "$A"
  $BUN r3-validate.ts animals/dairy-cow --volumes | tail -8
fi
