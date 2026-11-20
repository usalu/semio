#!/bin/sh
# 🔁 Rebuilds animals/hedgehog from the hand tables: rig and poses, both scales, the 2D redraw, the candidate document; validates it. Usage: sh r3-hedgehog-run.sh
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
"$BUN" r3-hedgehog-rig.ts | grep -E "WARN"
for s in 50 100 200 500; do "$BUN" r3-hedgehog-model.ts $s --write | tail -1; done
"$BUN" r3-hedgehog-draw2d.ts > "🗑️generated/r3-hedgehog/draw2d.txt" 2>&1 || tail -5 "🗑️generated/r3-hedgehog/draw2d.txt"
"$BUN" r3-hedgehog-candidate.ts
"$BUN" r3-validate.ts --file "🗑️generated/r3-hedgehog/🔣️.json" | cut -c1-300 | tail -${1:-25}
