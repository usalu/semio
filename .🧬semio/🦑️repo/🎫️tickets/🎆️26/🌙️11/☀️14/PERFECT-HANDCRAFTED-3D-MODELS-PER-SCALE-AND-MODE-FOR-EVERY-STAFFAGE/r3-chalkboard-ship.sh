#!/bin/sh
# 🚢 Round 3 chalkboard ship chain: solids, candidate, validation of the candidate, root members into the catalog, volumes splice, product check, whole-asset validation.
set -e
cd "$(dirname "$0")"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
CATALOG="/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🪑️objects/chalkboard/🔣️.json"
python3 r3-chalkboard-volumes.py
python3 r3-chalkboard-roots.py "$@"
"$BUN" r3-validate.ts --file 🗑️generated/r3-chalkboard/🔣️.json --volumes
python3 r3-chalkboard-roots.py --apply "$@"
python3 volumes-insert.py volumes/objects/chalkboard.json "$CATALOG"
"$BUN" volumes-check.ts objects/chalkboard --product | grep -v "^\[check\]" | grep -E "^(FAIL|PASS|ok   volumes-thermal)|findings|^  " | cut -c1-900
"$BUN" r3-validate.ts objects/chalkboard --volumes
"$BUN" r3-sliding-door-suit.ts objects/chalkboard
