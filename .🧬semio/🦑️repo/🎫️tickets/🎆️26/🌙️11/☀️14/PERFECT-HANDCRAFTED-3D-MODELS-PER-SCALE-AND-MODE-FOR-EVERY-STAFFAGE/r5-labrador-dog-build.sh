#!/bin/sh
# 🦮️ Round 5 Labrador: rebuilds rig, the four scales, the 2D redraw and the candidate from the hand tables and validates the candidate with its volumes. Usage: sh r5-labrador-dog-build.sh
cd "$(dirname "$0")"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
G="🗑️generated/r5-labrador-dog"
CAT="../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🐕️animals/labrador-dog/🔣️.json"
python3 r5-labrador-dog-rig.py "🗑️generated/r3-labrador-dog/asset-before.json" "$G/roots.json" | grep -E "^(REACH|LIMIT|pad )" || true
"$BUN" r5-labrador-dog-50.ts --write | head -1
for s in 100 200 500; do "$BUN" r5-labrador-dog-coarse.ts $s --write | grep -v "^/" | head -1; done
"$BUN" r5-labrador-dog-draw2d.ts | tail -1
python3 r5-labrador-dog-candidate.py "$CAT" "$G/roots.json" "$G/reps.json" volumes/animals/labrador-dog.json
"$BUN" r3-validate.ts --file "$G/🔣️.json" --volumes | cut -c1-400 | tail -12
