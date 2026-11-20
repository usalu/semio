#!/bin/sh
# 🔁 Rebuilds the work document of animals/roe-deer from the pre-round asset: rig and poses, the four scales, the 2D redraw, the dimensions; prints the validation. Usage: sh r3-roe-deer-run.sh [check]
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
W="🗑️generated/r3-roe-deer/🔣️.json"
python3 r3-roe-deer-rig.py --candidate --fresh | grep -E "WARN|candidate"
"$BUN" r3-roe-deer-50.ts --write | tail -1
for s in 100 200 500; do "$BUN" r3-roe-deer-coarse.ts $s --write | tail -1; done
"$BUN" r3-roe-deer-draw2d.ts > "🗑️generated/r3-roe-deer/draw2d.out" 2>&1 || tail -5 "🗑️generated/r3-roe-deer/draw2d.out"
python3 r3-roe-deer-apply.py reps && python3 r3-roe-deer-texts.py && python3 r3-roe-deer-apply.py volumes
python3 r3-roe-deer-apply.py dims $("$BUN" r3-roe-deer-bbox.ts)
"$BUN" r3-validate.ts --file "$W" | tail -25
"$BUN" r3-roe-deer-print.ts "$W" | grep -B1 -A3 false
if [ "$1" = "check" ]; then "$BUN" volumes-check.ts "$W" --product > "🗑️generated/r3-roe-deer/check.txt" 2>&1; grep -c "^ok" "🗑️generated/r3-roe-deer/check.txt"; grep "^FAIL" "🗑️generated/r3-roe-deer/check.txt" | sed -E 's/ info:[^ ]+//g' | cut -c1-1200; fi
