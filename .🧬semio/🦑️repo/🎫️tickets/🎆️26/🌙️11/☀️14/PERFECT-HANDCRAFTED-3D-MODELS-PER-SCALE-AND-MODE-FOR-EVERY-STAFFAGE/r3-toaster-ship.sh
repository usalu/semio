#!/bin/sh
# 🚢 Round 3, appliances/toaster: volumes source, candidate document, validation of the candidate with the measured volume rules and print suitability, then (only when valid) roots into the catalog, volumes splice and validation of the catalog file.
set -e
cd "$(dirname "$0")"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
CATALOG="../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🔌️appliances/toaster/🔣️.json"
python3 r3-toaster-model.py > /dev/null
python3 r3-toaster-roots.py > /dev/null
if ! OUT=$("$BUN" r3-validate.ts --file "🗑️generated/r3-toaster/🔣️.json" --volumes); then echo "$OUT" | tail -25; exit 1; fi
"$BUN" r3-toaster-suit.ts --file "🗑️generated/r3-toaster/🔣️.json" > /dev/null || { "$BUN" r3-toaster-suit.ts --file "🗑️generated/r3-toaster/🔣️.json"; exit 1; }
python3 r3-toaster-roots.py --apply | tail -1
python3 volumes-insert.py volumes/appliances/toaster.json "$CATALOG"
"$BUN" r3-validate.ts appliances/toaster --catalog-only --volumes | tail -5
