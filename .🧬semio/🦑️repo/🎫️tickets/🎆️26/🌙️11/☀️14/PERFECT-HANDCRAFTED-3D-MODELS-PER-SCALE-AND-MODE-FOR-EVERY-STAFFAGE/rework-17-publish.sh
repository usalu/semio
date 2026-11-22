#!/bin/sh
# 🚀 Author 17: publish the source of one appliance into the catalog and validate the catalog file alone.
cd "$(dirname "$0")"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
A=$(ls -d /Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/*/$1)/🔣️.json
python3 volumes-insert.py "volumes/appliances/$1.json" "$A" && $BUN r3-validate.ts "appliances/$1" --catalog-only --volumes 2>&1 | grep -v "^\[" | cut -c1-300 | tail -5
rm -f 🗑️generated/rework-17/$1.* 🗑️generated/r3-sheet/appliances/$1.* 🗑️generated/r3-2d/appliances/$1.*
