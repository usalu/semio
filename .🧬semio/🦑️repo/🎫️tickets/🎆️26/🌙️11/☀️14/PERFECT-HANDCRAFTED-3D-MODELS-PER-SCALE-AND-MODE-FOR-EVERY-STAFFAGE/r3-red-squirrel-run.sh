#!/bin/sh
# 🐿 Round 3 red squirrel: builds the candidate document from the hand tables and validates it; `catalog` then writes the root members to the catalog file and splices the volumes.
cd "$(dirname "$0")" || exit 1
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
C="🗑️generated/r3-red-squirrel/🔣️.json"
python3 r3-red-squirrel-build.py || exit 1
"$BUN" r3-validate.ts --file "$C" > "🗑️generated/r3-red-squirrel/val.txt"
awk '{print $1,$2}' "🗑️generated/r3-red-squirrel/val.txt" | sort | uniq -c | sort -rn | head -20
if [ "$1" = "catalog" ]; then
  tail -1 "🗑️generated/r3-red-squirrel/val.txt" | grep -q "^VALID" || { echo "candidate is not valid; catalog not written"; exit 1; }
  python3 r3-red-squirrel-build.py --apply || exit 1
  python3 volumes-insert.py volumes/animals/red-squirrel.json "$(ls -d /Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/*/red-squirrel)/🔣️.json"
  "$BUN" r3-validate.ts animals/red-squirrel
fi
