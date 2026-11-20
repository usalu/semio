#!/bin/sh
# 🦮️ Round 3 Labrador: the whole pipeline from the hand tables to a validated candidate (rig, volumes of every scale, 2D redraw, candidate validation); with `apply` the candidate's members are then written into the catalog asset and the volumes spliced. Usage: sh r3-labrador-dog-run.sh [apply]
set -e
T="$(cd "$(dirname "$0")" && pwd)"
cd "$T"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
CAT="$T/../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🐕️animals/labrador-dog/🔣️.json"
G="🗑️generated/r3-labrador-dog"
python3 r3-labrador-dog-rig.py "$G/asset-before.json" "$G/roots.json" | grep -E "^(REACH|LIMIT|pad|no findings)" || true
"$BUN" r3-labrador-dog-50.ts --write | head -2
for s in 100 200 500; do "$BUN" r3-labrador-dog-coarse.ts $s --write | grep -v "^/"; done
"$BUN" r3-labrador-dog-draw2d.ts | tail -1
python3 r3-labrador-dog-candidate.py "$CAT" "$G/roots.json" "$G/reps.json" volumes/animals/labrador-dog.json
"$BUN" r3-validate.ts --file "$G/🔣️.json" | cut -c1-600 | tail -25
if [ "$1" = "apply" ]; then
  "$BUN" r3-validate.ts --file "$G/🔣️.json" > /dev/null
  python3 r3-labrador-dog-apply.py "$CAT" "$G/roots.json" "$G/reps.json" volumes/animals/labrador-dog.json
  python3 volumes-insert.py volumes/animals/labrador-dog.json "$CAT"
  "$BUN" r3-validate.ts animals/labrador-dog | tail -1
fi
