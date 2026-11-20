#!/bin/sh
# 🚢 Round 3, vegetation/hosta-clump: build candidate and source, validate the candidate, write the root members into the catalog, splice the volumes, validate the catalog. Stops on the first failure.
set -e
cd "$(dirname "$0")"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
CAT="/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🌳️vegetation/hosta-clump/🔣️.json"
"$BUN" r3-hosta-clump-model.ts --source | cut -c1-300
"$BUN" r3-validate.ts --file "🗑️generated/r3-hosta-clump/🔣️.json" | tail -6 | cut -c1-400
"$BUN" r3-validate.ts --file "🗑️generated/r3-hosta-clump/🔣️.json" | tail -1 | grep -q "^VALID"
python3 r3-hosta-clump-roots.py "🗑️generated/r3-hosta-clump/🔣️.json" "$CAT" summary provenance rig anchors representations symbols poses clips thermal structure
python3 volumes-insert.py volumes/vegetation/hosta-clump.json "$CAT"
"$BUN" r3-validate.ts vegetation/hosta-clump | tail -3 | cut -c1-400
