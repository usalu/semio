#!/bin/sh
# 🚀 Author 10: full validation of one asset (schema, validateAsset, measured volume rules); on VALID publish the source into the catalog and validate the catalog file alone.
# Usage: sh rework-10-publish.sh <slug> [--allow-connected]
# --allow-connected publishes also when the ONLY findings left are `volumes-connected` ones (the rule landed during this rework and the catalog version fails it as well); the verdict line stays INVALID and is reported as such.
cd "$(dirname "$0")" || exit 2
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
A="machinery/$1"
ALL=$("$BUN" r3-validate.ts "$A" --volumes 2>&1 | grep -v '^\[')
echo "$ALL" | tail -14 | cut -c1-330
if echo "$ALL" | tail -1 | grep -q '^VALID '; then :
elif [ "$2" = "--allow-connected" ] && [ -z "$(echo "$ALL" | grep -v '^VOLUMES volumes-connected ' | grep -v '^INVALID ' | grep -v '^$')" ]; then echo "PUBLISHING WITH volumes-connected FINDINGS ONLY"
else echo "NOT PUBLISHED"; exit 1
fi
C=$(ls -d /Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/*/"$1")
python3 volumes-insert.py "volumes/$A.json" "$C/🔣️.json" && "$BUN" r3-validate.ts "$A" --catalog-only 2>&1 | tail -3 | cut -c1-300
