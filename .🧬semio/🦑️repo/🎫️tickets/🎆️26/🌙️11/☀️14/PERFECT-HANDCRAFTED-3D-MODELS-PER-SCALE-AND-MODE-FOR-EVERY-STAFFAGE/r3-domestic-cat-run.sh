#!/bin/sh
# 🔁 Rebuilds animals/domestic-cat from the hand tables: rig and poses, the three scales, the 2D redraw, the candidate document; validates it. Usage: sh r3-domestic-cat-run.sh
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
python3 r3-domestic-cat-rig.py | grep -E "WARN|LIMIT"
"$BUN" r3-domestic-cat-50.ts --write | tail -1
for s in 100 200; do "$BUN" r3-domestic-cat-coarse.ts $s --write | tail -1; done
"$BUN" r3-domestic-cat-draw2d.ts > "🗑️generated/r3-domestic-cat/draw2d.txt" 2>&1 || tail -5 "🗑️generated/r3-domestic-cat/draw2d.txt"
"$BUN" r3-domestic-cat-candidate.ts
"$BUN" r3-validate.ts --file "🗑️generated/r3-domestic-cat/🔣️.json" | cut -c1-500 | tail -25
