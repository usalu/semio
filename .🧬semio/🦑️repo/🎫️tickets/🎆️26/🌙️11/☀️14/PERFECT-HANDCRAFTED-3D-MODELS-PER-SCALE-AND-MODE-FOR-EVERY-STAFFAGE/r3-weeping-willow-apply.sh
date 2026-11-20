#!/bin/sh
# 🚿️ Rebuilds the round-3 weeping willow (model, drawings, candidate), validates the candidate and, only when it is VALID, writes the root members and splices the volumes into the catalog asset.
set -e
cd "$(dirname "$0")"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
CAT="../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🌳️vegetation/weeping-willow/🔣️.json"
C="🗑️generated/r3-weeping-willow/🔣️.json"
"$BUN" r3-weeping-willow-model.ts
"$BUN" r3-validate.ts --file "$C" | tail -6
"$BUN" r3-validate.ts --file "$C" > /dev/null
python3 r3-weeping-willow-roots.py "$C" "$CAT" rig anchors poses clips representations
python3 volumes-insert.py volumes/vegetation/weeping-willow.json "$CAT"
"$BUN" r3-validate.ts vegetation/weeping-willow | tail -1
