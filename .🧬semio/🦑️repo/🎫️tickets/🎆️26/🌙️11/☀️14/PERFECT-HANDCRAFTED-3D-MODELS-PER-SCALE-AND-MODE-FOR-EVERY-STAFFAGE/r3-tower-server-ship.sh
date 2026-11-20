#!/bin/sh
# 🚢 Round 3, appliances/tower-server: candidate document, validation of the candidate with the volume rules and print suitability, then (only when valid) roots into the catalog, volumes splice and validation of the catalog file.
set -e
cd "$(dirname "$0")"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
CATALOG="../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🔌️appliances/tower-server/🔣️.json"
python3 r3-tower-server-build.py
if ! OUT=$("$BUN" r3-validate.ts --file "🗑️generated/r3-tower-server/🔣️.json" --volumes); then echo "$OUT" | tail -25; exit 1; fi
"$BUN" r3-server-rack-suit.ts "🗑️generated/r3-tower-server/🔣️.json" 2>&1 | grep "^office" || true
python3 r3-tower-server-apply.py
python3 volumes-insert.py volumes/appliances/tower-server.json "$CATALOG"
"$BUN" r3-validate.ts appliances/tower-server --catalog-only --volumes | tail -5
