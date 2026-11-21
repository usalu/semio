#!/bin/sh
# 🚢 Round 5 of humans/courier-with-hand-truck: volumes, root candidate, 2D redraw, validation of the candidate; with --apply also writes the catalog (only when the candidate is VALID), splices the volumes and runs the product volume check and the full validation.
set -e
cd "$(dirname "$0")"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
C="../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/courier-with-hand-truck/🔣️.json"
O="🗑️generated/r5-courier-with-hand-truck"; mkdir -p "$O"
python3 r5-courier-with-hand-truck-build.py
$BUN r3-courier-roots.ts > /dev/null
$BUN r5-courier-with-hand-truck-draw2d.ts | tail -1
$BUN r3-validate.ts --file "🗑️generated/r3-courier-with-hand-truck/🔣️.json" > "$O/candidate.txt" || true
tail -1 "$O/candidate.txt"
[ "$1" = "--apply" ] || exit 0
grep -q "^VALID" "$O/candidate.txt" || { echo "candidate invalid: nothing written"; exit 1; }
python3 r3-courier-apply.py | tail -1
python3 volumes-insert.py volumes/humans/courier-with-hand-truck.json "$C" | tail -1
$BUN volumes-check.ts humans/courier-with-hand-truck --product > "$O/check.txt" 2>&1 || true
grep -o '![a-zA-Z.]*:[a-z-]*=[0-9.]*<=[0-9.]*' "$O/check.txt" || true
tail -1 "$O/check.txt"
$BUN r3-validate.ts humans/courier-with-hand-truck --volumes 2>&1 | tail -3 | cut -c1-200
