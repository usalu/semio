#!/bin/sh
# 🍲 Round 3 cooking pot ship chain: build the candidate, validate it, write roots and volumes into the catalog, validate and check the product rules.
set -e
cd "$(dirname "$0")"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
F="/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🪑️objects/cooking-pot/🔣️.json"
python3 r3-cooking-pot-build.py --source > /dev/null
"$BUN" r3-validate.ts --file "🗑️generated/r3-cooking-pot/🔣️.json" > "🗑️generated/r3-cooking-pot/validate.txt" || { tail -8 "🗑️generated/r3-cooking-pot/validate.txt"; exit 1; }
SCALES=50,100,200 "$BUN" r3-person-cooking-suit.ts "🗑️generated/r3-cooking-pot/🔣️.json" | cut -c1-200
python3 r3-cooking-pot-build.py --source --apply | tail -1
python3 volumes-insert.py volumes/objects/cooking-pot.json "$F" | tail -1
"$BUN" r3-validate.ts objects/cooking-pot | tail -3
"$BUN" volumes-check.ts objects/cooking-pot --product 2>&1 | grep -v "^\[check\]" | grep -v "^ok" | tail -20
