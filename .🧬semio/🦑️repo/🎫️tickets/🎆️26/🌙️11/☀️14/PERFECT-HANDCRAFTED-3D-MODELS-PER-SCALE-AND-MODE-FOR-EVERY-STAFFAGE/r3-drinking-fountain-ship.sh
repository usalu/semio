set -e
cd "$(dirname "$0")"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
C="/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🪑️objects/drinking-fountain/🔣️.json"
python3 r3-drinking-fountain-build.py
$BUN r3-validate.ts --file "🗑️generated/r3-drinking-fountain/🔣️.json" | tail -8
python3 r3-drinking-fountain-build.py --source --apply | tail -1
python3 volumes-insert.py volumes/objects/drinking-fountain.json "$C" | tail -1
$BUN volumes-check.ts objects/drinking-fountain --product 2>&1 | grep -v "^ok\|^\[check\]" | cut -c1-1800
