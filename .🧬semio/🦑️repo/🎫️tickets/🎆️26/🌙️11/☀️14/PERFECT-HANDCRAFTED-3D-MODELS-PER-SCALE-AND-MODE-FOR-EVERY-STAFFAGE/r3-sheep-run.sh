#!/bin/sh
# 🔁 Rebuilds the four scales of animals/sheep, redraws the 2D from them (two passes) and prints the failing lines of the product check. Usage: sh r3-sheep-run.sh [check]
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
"$BUN" r3-sheep-50.ts --write | tail -1
for s in 100 200 500; do "$BUN" r3-sheep-coarse.ts $s --write | tail -1; done
python3 r4-sheep-eye.py && "$BUN" r3-sheep-2d.ts 1 | tail -2 && python3 r3-sheep-2d.py && "$BUN" r3-sheep-2d.ts 2 | tail -2 && python3 r3-sheep-2d.py && "$BUN" r3-sheep-2d.ts 2 | tail -3 && python3 r3-sheep-2d.py
if [ "$1" = "check" ]; then "$BUN" volumes-check.ts animals/sheep --product > "🗑️generated/r3-sheep/check.txt" 2>&1; grep -E "^FAIL|^ok" "🗑️generated/r3-sheep/check.txt" | grep -c "^ok"; grep "^FAIL" "🗑️generated/r3-sheep/check.txt" | sed -E 's/ info:[^ ]+//g' | cut -c1-900; fi
