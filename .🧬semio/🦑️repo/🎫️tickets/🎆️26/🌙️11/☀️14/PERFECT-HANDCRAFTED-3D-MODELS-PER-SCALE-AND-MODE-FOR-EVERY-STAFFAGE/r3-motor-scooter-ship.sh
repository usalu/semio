#!/bin/sh
# 🚢 Ships the motor scooter: builds, validates the candidate, and only when it is VALID and print-suitable writes the root fixes to the catalog file, splices the volumes, validates the catalog file and runs the product volume check.
cd "$(dirname "$0")" || exit 1
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
C="🗑️generated/r3-motor-scooter/🔣️.json"
A="/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🚜️vehicles/motor-scooter/🔣️.json"
./r3-motor-scooter-run.sh | tail -6 | cut -c1-300
"$BUN" r3-validate.ts --file "$C" | tail -1 | grep -q "^VALID" || exit 1
"$BUN" r3-motor-scooter-suit.ts --file "$C" >/dev/null || exit 1
python3 r3-motor-scooter-roots.py --apply | tail -1
python3 volumes-insert.py volumes/vehicles/motor-scooter.json "$A" | tail -1
"$BUN" r3-validate.ts vehicles/motor-scooter --catalog-only | tail -3 | cut -c1-300
"$BUN" volumes-check.ts vehicles/motor-scooter --product 2>&1 | grep -v "^\[check\]" | awk '{ if ($1=="ok") print $1, $2, $3; else print substr($0,1,2500) }'
