#!/bin/sh
# 📦 Writes the work document of animals/roe-deer into the catalog asset (changed root members in one write), copies the volumes source and splices it with volumes-insert.py, then validates the catalog file and runs the product check. Usage: sh r3-roe-deer-publish.sh
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
F=$(ls /Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/*/roe-deer/🔣️.json)
"$BUN" r3-validate.ts --file "🗑️generated/r3-roe-deer/🔣️.json" | tail -1 | grep -q "^VALID" || { echo "work document invalid, nothing written"; exit 1; }
python3 r3-roe-deer-apply.py --catalog && cp volumes/adhoc/r3-roe-deer.json volumes/animals/roe-deer.json && python3 volumes-insert.py volumes/animals/roe-deer.json "$F" | tail -1
"$BUN" r3-validate.ts animals/roe-deer --catalog-only | tail -1
"$BUN" volumes-check.ts animals/roe-deer --product > "🗑️generated/r3-roe-deer/check-catalog.txt" 2>&1; grep -c "^ok" "🗑️generated/r3-roe-deer/check-catalog.txt"; grep "^FAIL" "🗑️generated/r3-roe-deer/check-catalog.txt" | cut -c1-600
