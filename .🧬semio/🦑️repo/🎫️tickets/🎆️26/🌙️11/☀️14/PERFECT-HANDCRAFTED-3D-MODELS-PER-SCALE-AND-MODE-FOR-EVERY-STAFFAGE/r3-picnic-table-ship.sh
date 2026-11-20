#!/bin/sh
# 🚢 Round 3, furniture/picnic-table: build the candidate, validate it, write roots and volumes into the catalog, validate the catalog.
set -e
cd "$(dirname "$0")"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
PY=~/Library/Caches/semio/staffage-oracles/geometry/bin/python3
C="../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🛋️furniture/picnic-table/🔣️.json"
$PY r3-picnic-table-build.py
$BUN r3-validate.ts --file "🗑️generated/r3-picnic-table/🔣️.json"
python3 r3-picnic-table-apply.py
python3 volumes-insert.py volumes/furniture/picnic-table.json "$C"
$BUN r3-validate.ts furniture/picnic-table
