#!/bin/sh
# 🔁 Build the candidate, validate it, write the catalog root members and the volumes source, splice and validate the catalog asset.
set -e
cd "$(dirname "$0")"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
python3 r3-heat-pump-outdoor-unit-kit.py
$BUN r3-validate.ts --file "🗑️generated/r3-heat-pump-outdoor-unit/🔣️.json" | tail -5
python3 r3-heat-pump-outdoor-unit-kit.py --apply
python3 volumes-insert.py volumes/machinery/heat-pump-outdoor-unit.json "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🏗️machinery/heat-pump-outdoor-unit/🔣️.json"
$BUN r3-validate.ts machinery/heat-pump-outdoor-unit | tail -5
