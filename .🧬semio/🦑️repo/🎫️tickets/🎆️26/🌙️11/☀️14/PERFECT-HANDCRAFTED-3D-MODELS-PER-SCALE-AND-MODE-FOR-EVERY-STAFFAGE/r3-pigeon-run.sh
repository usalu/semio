#!/bin/sh
# 🕊️ Round 3 pigeon: the whole chain. Bodies, thermal and structure of every scale, the 2D redraw from the bodies, the trial document, its validation. `sh r3-pigeon-run.sh catalog` then writes the catalog file and splices the volumes.
set -e
cd "$(dirname "$0")"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
for s in 50 100 200 500; do "$BUN" r3-pigeon-model.ts $s; done
"$BUN" r3-pigeon-2d.ts | tail -2
python3 r3-pigeon-apply.py | cut -c1-200
python3 r3-pigeon-full.py
"$BUN" r3-validate.ts --file "🗑️generated/r3-pigeon-full/🔣️.json" | cut -c1-260 | tail -${TAIL:-25}
if [ "$1" = "catalog" ]; then
  python3 r3-pigeon-apply.py --catalog | cut -c1-120
  python3 volumes-insert.py volumes/animals/pigeon.json "$(ls -d /Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/*/pigeon)/🔣️.json"
  "$BUN" r3-validate.ts animals/pigeon | tail -3
fi
