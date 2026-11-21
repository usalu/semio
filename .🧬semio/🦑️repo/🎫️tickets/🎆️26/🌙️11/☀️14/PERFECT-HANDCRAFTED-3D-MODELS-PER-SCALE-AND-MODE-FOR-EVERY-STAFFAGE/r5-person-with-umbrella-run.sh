#!/bin/sh
# 🔁 Rebuilds roots, volumes source, 2D shapes and the candidate asset of humans/person-with-umbrella and validates the candidate; `--commit` then writes the catalog (root members, volumes splice) and validates it.
cd "$(dirname "$0")" || exit 1
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
W=r5-person-with-umbrella
CAT=$(ls /Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/*/person-with-umbrella/🔣️.json)
"$BUN" $W-roots.ts | tail -1 && "$BUN" $W-build.ts || { echo "build failed, nothing written"; exit 1; }; "$BUN" $W-draw2d.ts > "🗑️generated/$W/draw2d.log" || { echo "draw2d failed, nothing written"; exit 1; }
python3 $W-apply.py > /dev/null && "$BUN" r3-validate.ts --file "🗑️generated/$W/🔣️.json" | tail -6
[ "$1" = "--commit" ] || exit 0
"$BUN" r3-validate.ts --file "🗑️generated/$W/🔣️.json" | tail -1 | grep -q "^VALID" || { echo "candidate invalid, nothing written"; exit 1; }
python3 $W-apply.py --write | tail -2 && python3 volumes-insert.py volumes/humans/person-with-umbrella.json "$CAT" | tail -1 && "$BUN" r3-validate.ts humans/person-with-umbrella --catalog-only | tail -1
