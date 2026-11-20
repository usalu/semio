#!/bin/sh
# 🔁 Build the forklift, validate the candidate, write the catalog root members, splice the volumes, validate the catalog; `check` adds the product volume check of every scale.
cd "$(dirname "$0")" || exit 1
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
A=machinery/forklift
C="/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🏗️machinery/forklift/🔣️.json"
G="🗑️generated/r3-forklift"
python3 r3-forklift-build.py || exit 1
$BUN r3-validate.ts --file "$G/🔣️.json" > "$G/validate.txt" 2>&1
tail -1 "$G/validate.txt" | cut -c1-200
if ! tail -1 "$G/validate.txt" | grep -q "^VALID"; then cut -c1-300 "$G/validate.txt" | head -30; exit 1; fi
python3 r3-forklift-build.py --apply | tail -1
python3 volumes-insert.py volumes/$A.json "$C" | tail -1
$BUN r3-validate.ts $A --catalog-only 2>&1 | tail -1 | cut -c1-200
[ "$1" = "check" ] || exit 0
$BUN volumes-check.ts $A --product > "$G/check.txt" 2>&1
grep -E "^(ok|FAIL|PASS)" "$G/check.txt" | cut -c1-90
tail -1 "$G/check.txt"
