#!/bin/sh
# 🐐️ Round 5 goat: the whole pipeline from the hand tables to the catalog asset (volumes of every scale, splice, 2D redraw, candidate validation, write, check). Usage: sh r5-goat-run.sh
set -e
T="$(cd "$(dirname "$0")" && pwd)"
cd "$T"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
CAT="$T/../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🐕️animals/goat/🔣️.json"
"$BUN" r5-goat-50.ts --write
for s in 100 200 500; do "$BUN" r5-goat-coarse.ts $s --write; done
python3 volumes-insert.py volumes/animals/goat.json "$CAT"
"$BUN" r3-goat-draw2d.ts > /dev/null
python3 r3-goat-candidate.py "$CAT" "🗑️generated/r3-goat/reps.json"
"$BUN" r3-validate.ts --file "🗑️generated/r3-goat/🔣️.json"
python3 r3-goat-apply.py "$CAT" "🗑️generated/r3-goat/reps.json"
"$BUN" r3-validate.ts animals/goat --volumes || true
for s in 50 100 200 500; do "$BUN" volumes-check.ts animals/goat --product --scale $s | tail -1; done
