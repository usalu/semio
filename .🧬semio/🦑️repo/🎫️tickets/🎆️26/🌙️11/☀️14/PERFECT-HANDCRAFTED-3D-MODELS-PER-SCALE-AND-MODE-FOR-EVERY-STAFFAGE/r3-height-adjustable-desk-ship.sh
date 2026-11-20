#!/bin/sh
# Round 3, furniture/height-adjustable-desk: build the candidate, validate it, write roots and volumes into the catalog.
set -e
cd "$(dirname "$0")"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
C="../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🛋️furniture/height-adjustable-desk/🔣️.json"
python3 r3-height-adjustable-desk-build.py
"$BUN" r3-validate.ts --file 🗑️generated/r3-height-adjustable-desk/🔣️.json --volumes | tail -12
python3 r3-height-adjustable-desk-apply.py
python3 volumes-insert.py volumes/furniture/height-adjustable-desk.json "$C"
