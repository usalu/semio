#!/bin/bash
# 🚢 Round 5 vehicles: validates the candidate (catalog + root patch + volumes source), and only when VALID writes the root patch, splices the volumes, validates the catalog with volumes and runs the product volume check. Usage: r5-vehicles-ship.sh <category/slug> <apply command...>
cd "$(dirname "$0")"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
A=$1; shift; SLUG=${A#*/}; CAT=${A%/*}
OUT="🗑️generated/r5-$SLUG"; mkdir -p "$OUT"
"$@" > /dev/null || exit 1
$BUN r3-validate.ts --file "$OUT/🔣️.json" > "$OUT/validate-candidate.txt" 2>&1
tail -1 "$OUT/validate-candidate.txt" | cut -c1-300
tail -1 "$OUT/validate-candidate.txt" | grep -q "^VALID" || { grep -v "^VALID" "$OUT/validate-candidate.txt" | cut -c1-300 | head; exit 1; }
"$@" --apply | tail -1
F=$(ls /Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/*/$SLUG/🔣️.json)
python3 volumes-insert.py "volumes/$CAT/$SLUG.json" "$F" | tail -1
$BUN r3-validate.ts $A --catalog-only --volumes > "$OUT/validate-catalog.txt" 2>&1; tail -1 "$OUT/validate-catalog.txt" | cut -c1-300
$BUN volumes-check.ts $A --product > "$OUT/check.txt" 2>&1; grep -v "^\[check\]\|^ok" "$OUT/check.txt" | cut -c1-400 | tail -15
