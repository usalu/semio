#!/bin/sh
# 🦆 Round 3 mallard: the whole chain. Roots into the trial document, bodies, thermal and structure of every scale, the 2D redraw from the bodies, the trial document with volumes, its validation. `sh r3-mallard-duck-run.sh catalog` then writes the catalog file and splices the volumes.
set -e
cd "$(dirname "$0")"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
python3 r3-mallard-duck-apply.py | cut -c1-200
for s in 50 100 200 500; do "$BUN" r3-mallard-duck-model.ts $s; done
if [ -f r3-mallard-duck-2d.ts ] && [ "$1" != "model" ]; then
  "$BUN" r3-mallard-duck-2d.ts | tail -${TAIL2D:-3}
  python3 r3-mallard-duck-apply.py | cut -c1-200
fi
python3 r3-mallard-duck-full.py
"$BUN" r3-validate.ts --file "🗑️generated/r3-mallard-duck-full/🔣️.json" | cut -c1-260 | tail -${TAIL:-25}
"$BUN" r3-mallard-duck-print.ts | grep -E "^1:" | tr '\n' ' '
echo
if "$BUN" r3-mallard-duck-print.ts | grep -q unsuitable; then echo "print: unsuitable, catalog not written"; exit 1; fi
if ! "$BUN" r3-validate.ts --file "🗑️generated/r3-mallard-duck-full/🔣️.json" | tail -1 | grep -q "^VALID"; then echo "trial document invalid, catalog not written"; exit 1; fi
if [ "$1" = "catalog" ]; then
  python3 r3-mallard-duck-apply.py --catalog | cut -c1-120
  python3 volumes-insert.py volumes/animals/mallard-duck.json "$(ls -d /Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/*/mallard-duck)/🔣️.json"
  "$BUN" r3-validate.ts animals/mallard-duck | tail -3
fi
