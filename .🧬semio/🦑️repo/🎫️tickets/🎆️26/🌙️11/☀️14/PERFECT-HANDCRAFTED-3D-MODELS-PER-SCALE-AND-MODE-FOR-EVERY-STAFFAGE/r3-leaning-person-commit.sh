#!/bin/sh
# 📌 Writes the validated candidate of humans/leaning-person into the catalog (root members, manifest row, then the volumes splice) and validates the catalog file.
cd "$(dirname "$0")" || exit 1
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
CAT="/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/leaning-person/🔣️.json"
"$BUN" r3-leaning-person-roots.ts > /dev/null && "$BUN" r3-validate.ts --file "🗑️generated/r3-leaning-person/🔣️.json" | tail -1 | grep -q "^VALID" || { echo "candidate invalid, nothing written"; exit 1; }
"$BUN" r3-leaning-person-roots.ts --commit | tail -1 && python3 volumes-insert.py volumes/humans/leaning-person.json "$CAT" | tail -1 && "$BUN" r3-validate.ts humans/leaning-person --catalog-only | tail -1
