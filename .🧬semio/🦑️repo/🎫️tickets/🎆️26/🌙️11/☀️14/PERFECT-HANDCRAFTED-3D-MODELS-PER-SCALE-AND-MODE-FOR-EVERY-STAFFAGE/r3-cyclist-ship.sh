#!/bin/sh
# 🚚 Rebuilds, validates and writes humans/cyclist into the catalog (document, then the volumes splice), then validates the catalog file.
set -e
cd "$(dirname "$0")"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
G="🗑️generated/r3-cyclist"
CAT="../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/cyclist/🔣️.json"
python3 r3-cyclist-root.py > "$G/root.log"
python3 r3-cyclist-build.py
python3 r3-cyclist-draw2d.py
"$BUN" r3-validate.ts --file "$G/🔣️.json" | tail -1 | grep -q "^VALID" || { echo "candidate invalid"; exit 1; }
"$BUN" r3-cyclist-suit.ts --file "$G/🔣️.json" || { echo "candidate not printable"; exit 1; }
"$BUN" r3-cyclist-apply.ts
python3 volumes-insert.py volumes/humans/cyclist.json "$CAT"
cp "$CAT" "$G/asset.applied.json"
"$BUN" r3-validate.ts humans/cyclist | tail -1
"$BUN" r3-validate.ts humans/cyclist --catalog-only | tail -1
