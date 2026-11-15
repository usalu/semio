#!/bin/bash
# 🧵 Author 64: splices the given sources (category/slug) into the catalog.
CAT="/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog"
cd "$(dirname "$0")"
for a in "$@"; do
  cat_="${a%%/*}"; slug="${a##*/}"
  f=$(ls "$CAT"/*"$cat_"/"$slug"/*.json | head -1)
  python3 volumes-insert.py "volumes/$a.json" "$f" && echo "SPLICED $a"
done
