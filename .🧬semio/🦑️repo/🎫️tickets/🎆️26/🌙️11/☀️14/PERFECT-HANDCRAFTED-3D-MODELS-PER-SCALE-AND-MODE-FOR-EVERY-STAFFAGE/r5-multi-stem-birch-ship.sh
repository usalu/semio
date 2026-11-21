#!/bin/sh
# 🚢 Round 5 birch: build, validate the candidate, write roots and splice only when valid, validate the catalog with volumes, run the product volume check.
set -e
cd "$(dirname "$0")"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
G="🗑️generated/r5-multi-stem-birch"
C="../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🌳️vegetation/multi-stem-birch/🔣️.json"
mkdir -p "$G"
python3 r5-multi-stem-birch-derive.py
"$BUN" r5-multi-stem-birch-model.ts > "$G/model.txt"
"$BUN" r3-validate.ts --file "$G/🔣️.json" > "$G/validate.txt" || { tail -8 "$G/validate.txt" | cut -c1-400; exit 1; }
tail -1 "$G/validate.txt"
"$BUN" r3-multi-stem-birch-roots.ts "$G/🔣️.json" "$C" rig anchors thermal representations symbols
python3 volumes-insert.py volumes/vegetation/multi-stem-birch.json "$C"
"$BUN" r3-validate.ts vegetation/multi-stem-birch --volumes > "$G/validate-catalog.txt" || true
tail -3 "$G/validate-catalog.txt" | cut -c1-400
"$BUN" volumes-check.ts vegetation/multi-stem-birch --product > "$G/check.txt" || true
grep -E "^!|FAIL|PASS|fail" "$G/check.txt" | cut -c1-300 | tail -25
