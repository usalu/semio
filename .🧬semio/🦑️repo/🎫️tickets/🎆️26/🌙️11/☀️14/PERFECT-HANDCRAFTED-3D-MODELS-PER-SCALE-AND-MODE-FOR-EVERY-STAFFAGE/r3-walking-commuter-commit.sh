#!/bin/sh
# 📌 Writes the validated candidate of humans/walking-commuter to the catalog (root members, then the volumes splice) and validates the catalog file.
cd "$(dirname "$0")" || exit 1
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
CAT=$(ls /Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/*/walking-commuter/🔣️.json)
"$BUN" r3-validate.ts --file "🗑️generated/r3-walking-commuter/🔣️.json" | tail -1 | grep -q "^VALID" || { echo "candidate invalid, nothing written"; exit 1; }
python3 r3-walking-commuter-root.py --commit | tail -2 && python3 volumes-insert.py volumes/humans/walking-commuter.json "$CAT" | tail -1 && "$BUN" r3-validate.ts humans/walking-commuter --catalog-only | tail -1
