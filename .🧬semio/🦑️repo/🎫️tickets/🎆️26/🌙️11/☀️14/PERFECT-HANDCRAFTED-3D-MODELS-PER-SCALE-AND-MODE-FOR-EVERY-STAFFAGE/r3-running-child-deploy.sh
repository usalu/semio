#!/bin/sh
# 🚀 Writes the validated round 3 state of humans/running-child into the catalog: rebuilds and validates the candidate, then roots, 2D geometry and texts (member-wise splice), the volumes (volumes-insert.py) and the manifest row; stops before the first write when the candidate is not valid.
cd "$(dirname "$0")" || exit 1
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
A="/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/running-child/🔣️.json"
rm -f "🗑️generated/r3-running-child/🔣️.json"
./r3-running-child-run.sh x 3 | tail -1 | grep -q "^VALID" || { echo "candidate is not valid"; exit 1; }
python3 r3-running-child-apply.py || exit 1
python3 volumes-insert.py volumes/humans/running-child.json "$A" || exit 1
python3 r3-running-child-manifest.py || exit 1
"$BUN" r3-validate.ts humans/running-child --catalog-only | cut -c1-400 | tail -5
