#!/bin/sh
# 🚢 Rebuilds, validates and writes humans/skateboarder into the catalog (document, then the volumes splice), then validates the catalog file.
set -e
cd "$(dirname "$0")"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
G="🗑️generated/r3-skateboarder"
CAT="../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/skateboarder/🔣️.json"
python3 r3-skateboarder-root.py > "$G/root.log"
python3 r3-skateboarder-build.py
python3 r3-skateboarder-draw2d.py
"$BUN" r3-validate.ts --file "$G/🔣️.json" | tail -1 | grep -q "^VALID" || { echo "candidate invalid"; exit 1; }
"$BUN" r3-skateboarder-suit.ts --file "$G/🔣️.json" || { echo "candidate not printable"; exit 1; }
"$BUN" r3-skateboarder-apply.ts
python3 volumes-insert.py volumes/humans/skateboarder.json "$CAT"
cp "$CAT" "$G/asset.applied.json"
"$BUN" r3-validate.ts humans/skateboarder | tail -1
"$BUN" r3-validate.ts humans/skateboarder --catalog-only | tail -1
