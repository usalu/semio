#!/bin/sh
# 🚢 Builds the round-3 bamboo candidate, validates it, writes the root members and splices the volumes into the catalog, validates the catalog; stops on the first failure.
set -e
cd "$(dirname "$0")"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
CAT="../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🌳️vegetation/bamboo-clump/🔣️.json"
mkdir -p "🗑️generated/r3-bamboo-clump"
"$BUN" r3-bamboo-clump-model.ts --source > "🗑️generated/r3-bamboo-clump/model.log"
grep "extent\|massKg\|body" "🗑️generated/r3-bamboo-clump/model.log"
"$BUN" r3-validate.ts --file "🗑️generated/r3-bamboo-clump/🔣️.json" | tail -3
"$BUN" r3-validate.ts --file "🗑️generated/r3-bamboo-clump/🔣️.json" | tail -1 | grep -q "^VALID"
python3 r3-bamboo-clump-roots.py
python3 volumes-insert.py volumes/vegetation/bamboo-clump.json "$CAT"
"$BUN" r3-validate.ts vegetation/bamboo-clump --catalog-only | tail -1
