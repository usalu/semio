#!/bin/sh
# 🚢 Builds the Lombardy poplar model, validates the candidate document and, only when it is valid, writes the root fixes and splices the volumes into the catalog asset.
set -e
cd "$(dirname "$0")"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
C="../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🌳️vegetation/lombardy-poplar/🔣️.json"
"$BUN" r3-lombardy-poplar-model.ts | grep -v "^1:.* shapes" || true
"$BUN" r3-validate.ts --file "🗑️generated/r3-lombardy-poplar/🔣️.json" | tail -3 | grep -q "^VALID" || { echo "candidate INVALID"; exit 1; }
python3 r3-lombardy-poplar-roots.py "🗑️generated/r3-lombardy-poplar/🔣️.json" "$C" rig anchors poses clips thermal representations symbols
python3 volumes-insert.py volumes/vegetation/lombardy-poplar.json "$C"
"$BUN" r3-validate.ts vegetation/lombardy-poplar --catalog-only | tail -3
