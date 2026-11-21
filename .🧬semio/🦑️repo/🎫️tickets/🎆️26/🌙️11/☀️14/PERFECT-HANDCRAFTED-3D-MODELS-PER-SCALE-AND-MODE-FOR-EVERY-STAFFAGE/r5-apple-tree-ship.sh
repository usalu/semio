#!/bin/sh
# 🚚 Round 5 apple tree: builds the clumped crown, validates the candidate, writes anchors and representations into the catalog, splices the volumes and validates the catalog.
set -e
cd "$(dirname "$0")"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
C="../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🌳️vegetation/apple-tree/🔣️.json"
O="🗑️generated/r5-apple-tree"
"$BUN" r5-apple-tree-model.ts > "$O/model.txt"
"$BUN" r3-validate.ts --file "$O/🔣️.json" > "$O/validate.txt" || { cut -c1-250 "$O/validate.txt" | tail -8; exit 1; }
python3 r3-apple-tree-roots.py "$O/🔣️.json" "$C" anchors representations
python3 volumes-insert.py volumes/vegetation/apple-tree.json "$C"
"$BUN" r3-validate.ts vegetation/apple-tree --catalog-only | tail -2
