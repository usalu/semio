#!/bin/sh
# 🔗 Author 09: validate one asset, publish it when VALID, then survey the next asset (floats and three sheets). Usage: sh rework-09-chain.sh <slug> [next-slug] [h]
cd "$(dirname "$0")" || exit 1
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
C="/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🚜️vehicles"
G="🗑️generated/rework-09"
"$BUN" r3-validate.ts "vehicles/$1" --volumes > "$G/$1.validate.log" 2>&1
if tail -1 "$G/$1.validate.log" | grep -q "^VALID"; then
  python3 volumes-insert.py "volumes/vehicles/$1.json" "$C/$1/🔣️.json" | tail -1 >> "$G/$1.validate.log"
  "$BUN" r3-validate.ts "vehicles/$1" --catalog-only 2>&1 | tail -1 >> "$G/$1.validate.log"
  echo PUBLISHED >> "$G/$1.validate.log"
fi
echo CHAINVALIDATED >> "$G/$1.validate.log"
if [ -n "$2" ]; then
  for s in 50 100 200 500; do for l in body structure; do "$BUN" rework-09-float.ts "vehicles/$2" --scale $s --list $l --h "${3:-0.015}" 2>&1 | awk '{print $1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12}'; done; done > "$G/$2.float.log" 2>&1
  "$BUN" r3-sheet.ts "vehicles/$2" --scale 50 > /dev/null 2>&1
  "$BUN" r3-sheet.ts "vehicles/$2" --scale 50 --list structure > /dev/null 2>&1
  "$BUN" r3-sheet.ts "vehicles/$2" --scale 500 > /dev/null 2>&1
  echo SURVEYED >> "$G/$2.float.log"
fi
