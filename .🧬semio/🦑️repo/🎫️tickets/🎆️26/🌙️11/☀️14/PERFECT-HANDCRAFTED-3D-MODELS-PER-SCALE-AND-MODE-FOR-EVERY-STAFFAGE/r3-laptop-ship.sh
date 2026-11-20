#!/bin/sh
# 🚢 Round 3, appliances/laptop: candidate document, validation of the candidate, then (only when valid) roots into the catalog, volumes splice and validation of the catalog file with print suitability.
set -e
cd "$(dirname "$0")"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
CATALOG="../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🔌️appliances/laptop/🔣️.json"
python3 r3-laptop-roots.py > /dev/null
if ! OUT=$("$BUN" r3-validate.ts --file "🗑️generated/r3-laptop/🔣️.json" --volumes); then echo "$OUT" | tail -25; exit 1; fi
"$BUN" r3-laptop-suit.ts --file "🗑️generated/r3-laptop/🔣️.json" > /dev/null || { "$BUN" r3-laptop-suit.ts --file "🗑️generated/r3-laptop/🔣️.json"; exit 1; }
python3 r3-laptop-apply.py
python3 volumes-insert.py volumes/appliances/laptop.json "$CATALOG"
"$BUN" r3-validate.ts appliances/laptop --catalog-only --volumes | tail -5
"$BUN" r3-laptop-suit.ts
