#!/bin/sh
# 🚢 Round 3, furniture/operating-table: builds the candidate, validates it, writes the catalog roots and the volumes source when valid, splices the volumes and validates the catalog asset.
set -e
cd "$(dirname "$0")"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
ASSET="../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🛋️furniture/operating-table/🔣️.json"
python3 r3-operating-table-build.py > /dev/null
$BUN r3-validate.ts --file "🗑️generated/r3-operating-table/🔣️.json" | cut -c1-500 | tail -14
$BUN r3-validate.ts --file "🗑️generated/r3-operating-table/🔣️.json" > /dev/null
python3 r3-operating-table-apply.py
python3 volumes-insert.py volumes/furniture/operating-table.json "$ASSET"
$BUN r3-validate.ts furniture/operating-table --catalog-only | tail -1
