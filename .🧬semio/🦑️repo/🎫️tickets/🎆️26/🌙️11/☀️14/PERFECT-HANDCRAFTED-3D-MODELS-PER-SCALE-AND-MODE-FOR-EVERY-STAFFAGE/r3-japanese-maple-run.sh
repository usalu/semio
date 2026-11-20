#!/bin/sh
# 🍁 Round-3 japanese maple: build the model and the candidate, validate it, write the roots and splice the volumes, validate the catalog.
set -e
cd "$(dirname "$0")"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
CAT="../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🌳️vegetation/japanese-maple/🔣️.json"
"$BUN" r3-japanese-maple-model.ts | grep -v '^1:[0-9]* \(elevation\|front\|plan\):' || true
"$BUN" r3-validate.ts --file "🗑️generated/r3-japanese-maple/🔣️.json" | tail -1 | grep -q '^VALID' || { "$BUN" r3-validate.ts --file "🗑️generated/r3-japanese-maple/🔣️.json" | tail -20; exit 1; }
"$BUN" r3-japanese-maple-roots.ts "🗑️generated/r3-japanese-maple/🔣️.json" "$CAT" rig anchors representations thermal symbols
python3 volumes-insert.py volumes/vegetation/japanese-maple.json "$CAT"
"$BUN" r3-validate.ts vegetation/japanese-maple --catalog-only | tail -1
