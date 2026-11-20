#!/bin/sh
# 🚢 Builds the round-3 miscanthus candidate, validates it, writes the root members and splices the volumes into the catalog, validates the catalog; stops on the first failure.
set -e
cd "$(dirname "$0")"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
CAT="../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🌳️vegetation/miscanthus-grass/🔣️.json"
mkdir -p "🗑️generated/r3-miscanthus-grass"
python3 r3-miscanthus-grass-model.py
"$BUN" r3-validate.ts --file "🗑️generated/r3-miscanthus-grass/🔣️.json" | tail -3
"$BUN" r3-validate.ts --file "🗑️generated/r3-miscanthus-grass/🔣️.json" | tail -1 | grep -q "^VALID"
python3 r3-miscanthus-grass-roots.py
python3 volumes-insert.py volumes/vegetation/miscanthus-grass.json "$CAT"
"$BUN" r3-validate.ts vegetation/miscanthus-grass --catalog-only | tail -1
