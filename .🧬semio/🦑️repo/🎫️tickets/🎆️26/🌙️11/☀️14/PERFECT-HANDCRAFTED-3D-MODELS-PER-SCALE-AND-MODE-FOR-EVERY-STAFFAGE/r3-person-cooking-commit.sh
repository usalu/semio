#!/bin/sh
# 📥 Writes humans/person-cooking into the catalog: rebuilds, stops unless the candidate is VALID, replaces the owned members, splices the volumes, validates the catalog asset.
set -e
cd "$(dirname "$0")"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
./r3-person-cooking-run.sh | tail -1 | grep -q '^VALID'
$BUN r3-person-cooking-roots.ts --commit | tail -1
python3 volumes-insert.py volumes/humans/person-cooking.json "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/person-cooking/🔣️.json"
$BUN r3-validate.ts humans/person-cooking --catalog-only | tail -3
