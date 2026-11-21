#!/bin/sh
# 🚢 Round 5, F03 furniture (single-bed, three-seat-sofa, bookshelf, operating-table): rebuilds the four volumes sources from the r5 build scripts, narrows the single bed's 1:200/1:500 print band boundary to 1:330 (the 30 mm side rail of the 1:200 plan is 0.191 mm at 1:350, under the 0.2 mm laser minimum), splices the volumes into the catalog and validates each catalog asset with its measured volume rules.
set -e
cd "$(dirname "$0")"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
CAT="../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🛋️furniture"
for A in single-bed three-seat-sofa bookshelf operating-table; do python3 "r5-$A-build.py"; done
python3 r5-furniture-range.py single-bed 200=150-330 500=330-1000 --apply
for A in single-bed three-seat-sofa bookshelf operating-table; do
  python3 volumes-insert.py "volumes/furniture/$A.json" "$CAT/$A/🔣️.json"
  $BUN r3-validate.ts "furniture/$A" --catalog-only --volumes | tail -1 | cut -c1-200
done
