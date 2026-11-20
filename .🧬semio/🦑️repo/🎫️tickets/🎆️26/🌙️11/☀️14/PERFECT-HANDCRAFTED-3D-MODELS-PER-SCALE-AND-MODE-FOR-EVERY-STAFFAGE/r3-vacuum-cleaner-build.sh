#!/bin/sh
# 🏭 Round 3, appliances/vacuum-cleaner: volumes source, candidate document, validation of the candidate, then (only when valid) roots into the catalog, volumes splice and validation of the catalog file.
set -e
cd "$(dirname "$0")"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
python3 r3-vacuum-cleaner-model.py > /dev/null
python3 r3-vacuum-cleaner-roots.py > /dev/null
if ! OUT=$("$BUN" r3-validate.ts --file "🗑️generated/r3-vacuum-cleaner/🔣️.json"); then echo "$OUT" | tail -25; exit 1; fi
echo "$OUT" | tail -1
python3 r3-vacuum-cleaner-apply.py
python3 volumes-insert.py volumes/appliances/vacuum-cleaner.json "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🔌️appliances/vacuum-cleaner/🔣️.json"
"$BUN" r3-validate.ts appliances/vacuum-cleaner --catalog-only | tail -25
"$BUN" r3-vacuum-cleaner-suit.ts
