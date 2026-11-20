#!/bin/sh
# 🚢 Ship chain of the round-3 ivy on trellis: model, validate the candidate, write the root members, splice the volumes, validate the catalog.
set -e
cd "$(dirname "$0")"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
A="../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🌳️vegetation/ivy-on-trellis/🔣️.json"
"$BUN" r3-ivy-on-trellis-model.ts | head -1
"$BUN" r3-validate.ts --file "🗑️generated/r3-ivy-on-trellis/🔣️.json" > "🗑️generated/r3-ivy-on-trellis/validate.txt" || { tail -8 "🗑️generated/r3-ivy-on-trellis/validate.txt"; exit 1; }
tail -1 "🗑️generated/r3-ivy-on-trellis/validate.txt"
python3 r3-ivy-on-trellis-roots.py "🗑️generated/r3-ivy-on-trellis/🔣️.json" "$A" rig anchors summary representations poses thermal
python3 volumes-insert.py volumes/vegetation/ivy-on-trellis.json "$A"
"$BUN" r3-validate.ts vegetation/ivy-on-trellis --catalog-only | tail -3
