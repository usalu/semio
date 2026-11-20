#!/bin/sh
# 🚢 Builds the Scots pine model, validates the candidate document and, only when it is valid, writes the root fixes and splices the volumes into the catalog asset.
set -e
cd "$(dirname "$0")"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
C="../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🌳️vegetation/scots-pine/🔣️.json"
"$BUN" r3-scots-pine-model.ts | grep -v "^1:.* shapes" || true
"$BUN" r3-validate.ts --file "🗑️generated/r3-scots-pine/🔣️.json" | tail -5
python3 r3-scots-pine-roots.py "🗑️generated/r3-scots-pine/🔣️.json" "$C" rig anchors representations thermal symbols
python3 volumes-insert.py volumes/vegetation/scots-pine.json "$C"
"$BUN" r3-validate.ts vegetation/scots-pine --catalog-only | tail -3
