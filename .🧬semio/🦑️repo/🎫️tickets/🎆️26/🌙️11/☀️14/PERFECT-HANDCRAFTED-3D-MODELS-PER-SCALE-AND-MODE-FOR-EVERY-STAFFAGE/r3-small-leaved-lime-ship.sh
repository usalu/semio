#!/bin/sh
# 🚢 Builds the small-leaved lime model, validates the candidate document and, only when it is valid, writes the root fixes and splices the volumes into the catalog asset.
set -e
cd "$(dirname "$0")"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
C="../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🌳️vegetation/small-leaved-lime/🔣️.json"
"$BUN" r3-small-leaved-lime-model.ts > "🗑️generated/r3-small-leaved-lime/model.txt"
grep -v "elevation:\|front:\|plan:" "🗑️generated/r3-small-leaved-lime/model.txt" || true
"$BUN" r3-validate.ts --file "🗑️generated/r3-small-leaved-lime/🔣️.json" > "🗑️generated/r3-small-leaved-lime/validate.txt" || { tail -8 "🗑️generated/r3-small-leaved-lime/validate.txt" | cut -c1-400; exit 1; }
tail -1 "🗑️generated/r3-small-leaved-lime/validate.txt"
python3 r3-small-leaved-lime-roots.py "🗑️generated/r3-small-leaved-lime/🔣️.json" "$C" rig anchors representations thermal symbols
python3 volumes-insert.py volumes/vegetation/small-leaved-lime.json "$C"
"$BUN" r3-validate.ts vegetation/small-leaved-lime --catalog-only | tail -3
