#!/bin/sh
# 🚀 Builds the candidate of humans/standing-adult, validates it and, only when it is valid, writes the root fixes and the volumes into the catalog asset.
set -e
cd "$(dirname "$0")"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
A="../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/standing-adult/🔣️.json"
python3 r3-standing-adult-build.py | tail -5
"$BUN" r3-validate.ts --file "🗑️generated/r3-standing-adult/🔣️.json" --volumes | tail -8
"$BUN" r3-validate.ts --file "🗑️generated/r3-standing-adult/🔣️.json" | grep -q "^VALID"
python3 r3-standing-adult-roots.py --apply
python3 volumes-insert.py volumes/humans/standing-adult.json "$A"
"$BUN" r3-validate.ts humans/standing-adult --catalog-only | tail -1
"$BUN" r3-standing-adult-suit.ts | cut -c1-60
