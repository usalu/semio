#!/bin/sh
# 🌳 Round-3 multi-stem birch: build the model and the candidate, validate it, write the roots and splice the volumes, validate the catalog.
set -e
cd "$(dirname "$0")"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
CAT="../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🌳️vegetation/multi-stem-birch/🔣️.json"
"$BUN" r3-multi-stem-birch-model.ts | grep '^1:[0-9]* body' || true
"$BUN" r3-validate.ts --file "🗑️generated/r3-multi-stem-birch/🔣️.json" | tail -1 | grep -q '^VALID' || { "$BUN" r3-validate.ts --file "🗑️generated/r3-multi-stem-birch/🔣️.json" | tail -20; exit 1; }
[ "$1" = "--dry" ] && exit 0
"$BUN" r3-multi-stem-birch-roots.ts "🗑️generated/r3-multi-stem-birch/🔣️.json" "$CAT" rig anchors thermal representations symbols
python3 volumes-insert.py volumes/vegetation/multi-stem-birch.json "$CAT"
"$BUN" r3-validate.ts vegetation/multi-stem-birch --catalog-only | tail -1
