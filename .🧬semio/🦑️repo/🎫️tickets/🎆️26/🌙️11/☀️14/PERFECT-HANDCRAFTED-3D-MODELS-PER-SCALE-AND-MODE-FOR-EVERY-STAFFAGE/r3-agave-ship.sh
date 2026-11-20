#!/bin/sh
# 🚢 Round 3, vegetation/agave: build candidate and source, validate the candidate, write the root members into the catalog, splice the volumes, validate the catalog. Stops on the first failure.
set -e
cd "$(dirname "$0")"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
CAT="/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🌳️vegetation/agave/🔣️.json"
"$BUN" r3-agave-model.ts --source | grep -v "^\[DEBUG\] rig:"
"$BUN" r3-validate.ts --file "🗑️generated/r3-agave/🔣️.json" | tail -3
"$BUN" r3-validate.ts --file "🗑️generated/r3-agave/🔣️.json" | tail -1 | grep -q "^VALID"
python3 r3-agave-roots.py "🗑️generated/r3-agave/🔣️.json" "$CAT" summary provenance rig anchors representations poses clips thermal structure
python3 volumes-insert.py volumes/vegetation/agave.json "$CAT"
"$BUN" r3-validate.ts vegetation/agave | tail -3
