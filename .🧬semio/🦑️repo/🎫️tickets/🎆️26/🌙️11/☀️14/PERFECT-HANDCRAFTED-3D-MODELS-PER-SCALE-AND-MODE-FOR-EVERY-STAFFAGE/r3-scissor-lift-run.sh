#!/bin/sh
# 🔁 Build, validate the candidate, write the catalog members, splice the volumes, validate the catalog; then the product check summary.
cd "$(dirname "$0")" || exit 1
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
A=machinery/scissor-lift
C="/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🏗️machinery/scissor-lift/🔣️.json"
python3 r3-scissor-lift-build.py || exit 1
$BUN r3-validate.ts --file "🗑️generated/r3-scissor-lift/🔣️.json" > "🗑️generated/r3-scissor-lift/validate.txt" 2>&1
tail -1 "🗑️generated/r3-scissor-lift/validate.txt" | cut -c1-200
if ! tail -1 "🗑️generated/r3-scissor-lift/validate.txt" | grep -q "^VALID"; then cut -c1-300 "🗑️generated/r3-scissor-lift/validate.txt" | head -30; exit 1; fi
python3 r3-scissor-lift-build.py --apply | tail -1
python3 volumes-insert.py volumes/$A.json "$C" | tail -1
$BUN r3-validate.ts $A --catalog-only 2>&1 | tail -1 | cut -c1-200
[ "$1" = "check" ] || exit 0
$BUN volumes-check.ts $A --product > "🗑️generated/r3-scissor-lift/check.txt" 2>&1
python3 r3-scissor-lift-summary.py
