#!/bin/sh
# 🔁 Round 5: rebuilds the four scales of animals/sheep from the r5 hand tables, redraws the 2D from them (two passes, ruling E1) and prints the failing lines of the product check. Usage: sh r5-sheep-run.sh [check]
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
"$BUN" r5-sheep-50.ts --write | tail -1
for s in 100 200 500; do "$BUN" r5-sheep-coarse.ts $s --write | tail -1; done
python3 r4-sheep-eye.py && "$BUN" r3-sheep-2d.ts 1 | tail -2 && python3 r3-sheep-2d.py && "$BUN" r3-sheep-2d.ts 2 | tail -2 && python3 r3-sheep-2d.py && "$BUN" r3-sheep-2d.ts 2 | tail -3 && python3 r3-sheep-2d.py
"$BUN" r3-validate.ts animals/sheep | tail -4
if [ "$1" = "check" ]; then "$BUN" volumes-check.ts animals/sheep --product > "🗑️generated/r5-sheep/check.txt" 2>&1; grep -c "^ok" "🗑️generated/r5-sheep/check.txt"; grep "^FAIL" "🗑️generated/r5-sheep/check.txt" | sed -E 's/ info:[^ ]+//g' | cut -c1-900; fi
