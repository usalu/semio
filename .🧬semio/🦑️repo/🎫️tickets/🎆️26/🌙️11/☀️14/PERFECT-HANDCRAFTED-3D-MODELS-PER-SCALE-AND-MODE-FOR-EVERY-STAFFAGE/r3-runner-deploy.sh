#!/bin/sh
# 🚀 Rebuilds the candidate of humans/runner-with-blade-prosthesis and, only when `validateAsset` and the schema report nothing, writes its members, the volumes and the manifest row into the catalog.
cd "$(dirname "$0")" || exit 1
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
A="/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/runner-with-blade-prosthesis/🔣️.json"
sh r3-runner-all.sh | grep "^ASSET\|^SCHEMA\|VALID\|VOLUMES" | cut -c1-300
"$BUN" r3-validate.ts --file "🗑️generated/r3-runner-with-blade-prosthesis/🔣️.json" | tail -1 | grep -q "^VALID" || { echo "NOT DEPLOYED"; exit 1; }
python3 r3-runner-splice.py summary dimensions rig anchors representations poses clips presentation > /dev/null && python3 volumes-insert.py volumes/humans/runner-with-blade-prosthesis.json "$A" && python3 r3-runner-splice.py --manifest "$("$BUN" r3-runner-power.ts | head -1)" > /dev/null && "$BUN" r3-validate.ts humans/runner-with-blade-prosthesis --catalog-only
