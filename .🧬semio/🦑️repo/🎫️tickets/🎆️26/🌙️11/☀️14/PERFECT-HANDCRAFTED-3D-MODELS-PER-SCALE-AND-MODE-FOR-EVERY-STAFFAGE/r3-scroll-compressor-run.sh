#!/bin/sh
# 🔁 Build the scroll compressor candidate, validate it, write the catalog root members and the volumes source, check, splice and validate the catalog asset.
set -e
cd "$(dirname "$0")"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
python3 r3-scroll-compressor-kit.py
$BUN r3-validate.ts --file "🗑️generated/r3-scroll-compressor/🔣️.json" | tail -3
python3 r3-scroll-compressor-kit.py --apply
$BUN volumes-check.ts machinery/scroll-compressor --product | tail -1
python3 volumes-insert.py volumes/machinery/scroll-compressor.json "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🏗️machinery/scroll-compressor/🔣️.json"
$BUN r3-validate.ts machinery/scroll-compressor --catalog-only --volumes | tail -3
