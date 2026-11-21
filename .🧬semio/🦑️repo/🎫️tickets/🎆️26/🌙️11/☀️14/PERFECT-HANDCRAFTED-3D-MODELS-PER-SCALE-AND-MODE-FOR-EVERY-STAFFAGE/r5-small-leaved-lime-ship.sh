#!/bin/sh
# 🚢 Round 5 lime: build, validate the candidate, write roots and splice only when valid, validate the catalog with volumes, run the product volume check.
set -e
cd "$(dirname "$0")"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
G="🗑️generated/r5-small-leaved-lime"
C="../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🌳️vegetation/small-leaved-lime/🔣️.json"
mkdir -p "$G"
python3 r5-small-leaved-lime-derive.py
"$BUN" r5-small-leaved-lime-model.ts > "$G/model.txt"
"$BUN" r3-validate.ts --file "$G/🔣️.json" > "$G/validate.txt" || { tail -8 "$G/validate.txt" | cut -c1-400; exit 1; }
tail -1 "$G/validate.txt"
python3 r3-small-leaved-lime-roots.py "$G/🔣️.json" "$C" rig anchors representations thermal symbols
python3 volumes-insert.py volumes/vegetation/small-leaved-lime.json "$C"
"$BUN" r3-validate.ts vegetation/small-leaved-lime --volumes > "$G/validate-catalog.txt" || true
tail -3 "$G/validate-catalog.txt" | cut -c1-400
"$BUN" volumes-check.ts vegetation/small-leaved-lime --product > "$G/check.txt" || true
grep -E "^!|FAIL|PASS|fail" "$G/check.txt" | cut -c1-300 | tail -25
