#!/bin/sh
# 🚀 Publishes one volumes source of the machinery rework into the catalog and proves it: `sh rework-machinery-publish.sh <category>/<slug>` (splice, full proof of the catalog file, product check CLI).
cd "$(dirname "$0")" || exit 2
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
S=/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage
SLUG=${1#*/}
FILE=$(ls -d "$S"/🗂️catalog/*/"$SLUG"/🔣️.json)
python3 volumes-insert.py "volumes/$1.json" "$FILE" | tail -1
"$BUN" r3-validate.ts "$1" --catalog-only --volumes 2>&1 | tail -4 | cut -c1-600
cd "$S" && "$BUN" 🔨️modules/🗂️catalog/🩺️check/🟦️.ts --asset "$SLUG" 2>&1 | grep -E "✓|✗|rror" | head -5
