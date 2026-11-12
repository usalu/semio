#!/bin/sh
# usage: run.sh <slug> [insert]  — author, check (draft file), optionally splice, validate, pose-check
cd "$(dirname "$0")/.." || exit 1
T=$(pwd)
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
SLUG=$1
FILE=$(ls -d /Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/*appliances/"$SLUG"/🔣️.json)
OUT="$T/🗑️generated/authoring-appliances-a"
mkdir -p "$OUT"
PYS=$(echo "$SLUG" | tr - _)
python3 authoring-appliances-a/"$PYS".py || exit 1
if [ "$2" = "insert" ]; then
  python3 volume-insert.py "volumes/appliances/$SLUG.json" "$FILE" || exit 1
  $BUN volume-check.ts "$SLUG" | tee "$OUT/$SLUG.check.txt"
  $BUN authoring-appliances-a/validate.ts "$SLUG"
  $BUN authoring-appliances-a/posecheck.ts "$SLUG"
else
  $BUN volume-check.ts "$SLUG" --volume "volumes/appliances/$SLUG.json" | tee "$OUT/$SLUG.draftcheck.txt" | grep -v "^    solids outside\|^    drawn shapes" 
  [ "$2" = "full" ] && grep "^    " "$OUT/$SLUG.draftcheck.txt"
fi
