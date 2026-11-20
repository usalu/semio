#!/bin/sh
# 🚢 Rebuilds, validates and writes humans/child-on-kick-scooter into the catalog (document, then the volumes splice), then validates the catalog file.
set -e
cd "$(dirname "$0")"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
G="🗑️generated/r3-child-on-kick-scooter"
CAT="../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/child-on-kick-scooter/🔣️.json"
./r3-child-on-kick-scooter-run.sh | tail -1 | grep -q "^VALID" || { echo "candidate invalid"; exit 1; }
"$BUN" r3-child-on-kick-scooter-probe.ts | tail -1 | grep -q "^0 failing" || { echo "probe failed"; exit 1; }
"$BUN" r3-child-on-kick-scooter-apply.ts
python3 volumes-insert.py volumes/humans/child-on-kick-scooter.json "$CAT"
cp "$CAT" "$G/asset.applied.json"
"$BUN" r3-validate.ts humans/child-on-kick-scooter | tail -1
"$BUN" r3-validate.ts humans/child-on-kick-scooter --catalog-only | tail -1
