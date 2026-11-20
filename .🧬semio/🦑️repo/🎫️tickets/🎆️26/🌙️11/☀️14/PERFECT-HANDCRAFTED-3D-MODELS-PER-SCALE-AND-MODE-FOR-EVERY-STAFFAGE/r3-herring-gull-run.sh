#!/bin/sh
# 🕊️ Round 3 herring gull: the whole chain. Roots into the trial document, bodies, thermal and structure of every scale, the 2D redraw from the bodies, the trial document with volumes, its validation. `sh r3-herring-gull-run.sh catalog` then writes the catalog file and splices the volumes.
set -e
cd "$(dirname "$0")"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
python3 r3-herring-gull-apply.py | cut -c1-200
for s in 50 100 200 500; do "$BUN" r3-herring-gull-model.ts $s; done
"$BUN" r3-herring-gull-2d.ts | tail -${TAIL2D:-3}
python3 r3-herring-gull-apply.py | cut -c1-200
python3 r3-herring-gull-full.py
"$BUN" r3-validate.ts --file "🗑️generated/r3-herring-gull-full/🔣️.json" | cut -c1-260 | tail -${TAIL:-25}
if ! "$BUN" r3-validate.ts --file "🗑️generated/r3-herring-gull-full/🔣️.json" | tail -1 | grep -q "^VALID"; then echo "trial document invalid, catalog not written"; exit 1; fi
if [ "$1" = "catalog" ]; then
  python3 r3-herring-gull-apply.py --catalog | cut -c1-120
  python3 volumes-insert.py volumes/animals/herring-gull.json "$(ls -d /Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/*/herring-gull)/🔣️.json"
  "$BUN" r3-validate.ts animals/herring-gull | tail -3
fi
