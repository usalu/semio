#!/bin/sh
# 🚜 Rebuilds everything of humans/kneeling-gardener and writes it: candidate (roots, volumes, 2D), validation of the candidate, then the catalog members and the volumes splice. Stops before writing when the candidate is not valid.
set -e
cd "$(dirname "$0")"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
./r3-kneeling-gardener-run.sh
"$BUN" r3-validate.ts --file "🗑️generated/r3-kneeling-gardener/🔣️.json" | tail -4
"$BUN" r3-kneeling-gardener-suit.ts 50 | grep -c false | grep -q "^0$"
"$BUN" r3-kneeling-gardener-suit.ts 100 | grep -c false | grep -q "^0$"
python3 r3-kneeling-gardener-write.py > /dev/null
python3 volumes-insert.py volumes/humans/kneeling-gardener.json "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/kneeling-gardener/🔣️.json" | tail -1
"$BUN" r3-validate.ts humans/kneeling-gardener --catalog-only | tail -2
