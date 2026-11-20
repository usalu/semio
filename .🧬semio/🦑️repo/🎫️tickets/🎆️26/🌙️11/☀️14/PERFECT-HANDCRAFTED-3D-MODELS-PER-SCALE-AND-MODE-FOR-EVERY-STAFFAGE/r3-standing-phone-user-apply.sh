#!/bin/sh
# 🚀 Builds the candidate of humans/standing-phone-user, validates it and, only when it is valid, writes the root fixes and the volumes into the catalog asset.
set -e
cd "$(dirname "$0")"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
A="../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/standing-phone-user/🔣️.json"
python3 r3-standing-phone-user-build.py | tail -5
"$BUN" r3-validate.ts --file "🗑️generated/r3-standing-phone-user/🔣️.json" --volumes | tail -8
"$BUN" r3-validate.ts --file "🗑️generated/r3-standing-phone-user/🔣️.json" | grep -q "^VALID"
python3 r3-standing-phone-user-roots.py --apply
python3 volumes-insert.py volumes/humans/standing-phone-user.json "$A"
"$BUN" r3-validate.ts humans/standing-phone-user --catalog-only | tail -1
"$BUN" r3-standing-phone-user-suit.ts | cut -c1-60
