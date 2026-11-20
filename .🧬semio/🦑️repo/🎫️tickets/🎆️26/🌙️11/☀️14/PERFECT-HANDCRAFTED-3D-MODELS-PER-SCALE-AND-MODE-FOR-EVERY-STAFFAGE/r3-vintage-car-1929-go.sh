#!/bin/sh
# 🔁️ One iteration of the round 3 vintage saloon: volumes source, candidate document (roots plus volumes), validation, root apply, splice. Usage: sh r3-vintage-car-1929-go.sh
cd "$(dirname "$0")" || exit 1
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
CAT="/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🚜️vehicles/vintage-car-1929/🔣️.json"
python3 r3-vintage-car-1929.py | cut -c1-8 || exit 1
python3 r3-vintage-car-1929-root.py | tail -1 | cut -c1-12 || exit 1
$BUN r3-validate.ts --file "🗑️generated/r3-vintage-car-1929/🔣️.json" 2>&1 | tail -6 | cut -c1-400
$BUN r3-validate.ts --file "🗑️generated/r3-vintage-car-1929/🔣️.json" >/dev/null 2>&1 || exit 1
python3 r3-vintage-car-1929-root.py --apply | tail -1 | cut -c1-8
python3 volumes-insert.py volumes/vehicles/vintage-car-1929.json "$CAT" | tail -2 | cut -c1-200
$BUN r3-validate.ts vehicles/vintage-car-1929 --catalog-only | tail -1 | cut -c1-200
$BUN r3-vintage-car-1929-suit.ts
