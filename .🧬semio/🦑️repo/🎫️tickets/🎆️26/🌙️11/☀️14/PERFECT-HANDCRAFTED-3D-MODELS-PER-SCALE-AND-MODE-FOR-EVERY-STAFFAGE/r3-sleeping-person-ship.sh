#!/bin/sh
# 🚢 Rebuilds and validates the candidate of humans/sleeping-person; when it is VALID writes the root members into the catalog, splices the volumes, validates the catalog file and runs the product volume check (output under 🗑️generated/r3-sleeping-person/).
set -e
cd "$(dirname "$0")"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
A="/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/sleeping-person/🔣️.json"
./r3-sleeping-person-run.sh > "🗑️generated/r3-sleeping-person/run.txt" 2>&1
tail -1 "🗑️generated/r3-sleeping-person/run.txt" | grep -q "^VALID" || { tail -20 "🗑️generated/r3-sleeping-person/run.txt"; exit 1; }
"$BUN" r3-sleeping-person-suit.ts
python3 r3-sleeping-person-write.py
python3 volumes-insert.py volumes/humans/sleeping-person.json "$A" | tail -1
"$BUN" r3-validate.ts humans/sleeping-person --catalog-only | tail -1
"$BUN" volumes-check.ts humans/sleeping-person --product > "🗑️generated/r3-sleeping-person/check.txt" 2>&1 || true
grep -v "^ok\|^\[check\]" "🗑️generated/r3-sleeping-person/check.txt" | sed 's/info:[^ ]*//g' | cut -c1-700
