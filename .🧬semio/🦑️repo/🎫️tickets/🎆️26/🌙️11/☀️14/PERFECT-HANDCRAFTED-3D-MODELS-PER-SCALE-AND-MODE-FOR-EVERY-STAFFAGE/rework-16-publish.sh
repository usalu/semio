#!/bin/sh
# Publishes one appliance source into the catalog and validates the catalog file: rework-16-publish.sh <slug>
cd "$(dirname "$0")" || exit 1
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
python3 volumes-insert.py "volumes/appliances/$1.json" "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🔌️appliances/$1/🔣️.json" || exit 1
"$BUN" r3-validate.ts "appliances/$1" --catalog-only | tail -4
