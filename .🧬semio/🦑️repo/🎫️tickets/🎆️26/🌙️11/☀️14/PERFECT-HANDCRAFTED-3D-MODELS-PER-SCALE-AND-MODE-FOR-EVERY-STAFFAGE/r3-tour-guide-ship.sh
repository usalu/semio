#!/bin/sh
# 🚢 Rebuilds the volumes source and the redrawn 2D of humans/tour-guide, validates the candidate document and, when it is valid, writes the roots to the catalog file and splices the volumes.
set -e
cd "$(dirname "$0")"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
$BUN r3-tour-guide-roots.ts > /dev/null
$BUN r3-tour-guide-build.ts
$BUN r3-tour-guide-draw2d.ts | grep -v "shapes," || true
python3 r3-tour-guide-apply.py > /dev/null
$BUN r3-validate.ts --file "🗑️generated/r3-tour-guide/🔣️.json" | cut -c1-400 | tail -8
$BUN r3-validate.ts --file "🗑️generated/r3-tour-guide/🔣️.json" > /dev/null
A=$(ls /Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/*/tour-guide/🔣️.json)
python3 r3-tour-guide-apply.py --write > /dev/null
python3 volumes-insert.py volumes/humans/tour-guide.json "$A" | tail -1
$BUN r3-validate.ts humans/tour-guide --catalog-only | tail -1
