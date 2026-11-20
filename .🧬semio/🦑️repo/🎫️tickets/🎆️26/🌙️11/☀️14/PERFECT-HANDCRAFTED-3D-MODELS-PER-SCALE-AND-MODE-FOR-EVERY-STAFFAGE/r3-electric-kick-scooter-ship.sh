#!/bin/sh
# 🚢 Ships the scooter: builds, validates the candidate, and only when it is VALID writes the root fixes to the catalog file, splices the volumes, validates the catalog file and runs the product volume check.
cd "$(dirname "$0")" || exit 1
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
C="🗑️generated/r3-electric-kick-scooter/🔣️.json"
A="/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🚜️vehicles/electric-kick-scooter/🔣️.json"
python3 r3-electric-kick-scooter-build.py && python3 r3-electric-kick-scooter-roots.py >/dev/null || exit 1
"$BUN" r3-validate.ts --file "$C" | tail -8 | cut -c1-300
"$BUN" r3-validate.ts --file "$C" | tail -1 | grep -q "^VALID" || exit 1
"$BUN" r3-electric-kick-scooter-suit.ts --file "$C" || exit 1
python3 r3-electric-kick-scooter-roots.py --apply | tail -1
python3 volumes-insert.py volumes/vehicles/electric-kick-scooter.json "$A" | tail -1
"$BUN" r3-validate.ts vehicles/electric-kick-scooter --catalog-only | tail -3 | cut -c1-300
"$BUN" volumes-check.ts vehicles/electric-kick-scooter --product 2>&1 | grep -v "^\[check\]" | awk '{ if ($1=="ok") print $1, $2, $3; else print substr($0,1,2500) }'
