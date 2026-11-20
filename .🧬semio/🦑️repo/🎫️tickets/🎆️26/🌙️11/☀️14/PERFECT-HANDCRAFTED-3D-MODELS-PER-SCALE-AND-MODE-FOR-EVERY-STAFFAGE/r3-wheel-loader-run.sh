#!/bin/sh
# 🔁 Round-3 wheel loader loop: build the candidate, validate it, write the roots to the catalog, splice the volumes and print the failing measurements of the product check (all scales, or the scales given).
cd "$(dirname "$0")" || exit 1
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
A="/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🏗️machinery/wheel-loader/🔣️.json"
G="🗑️generated/r3-wheel-loader"
python3 r3-wheel-loader-build.py | head -1 || exit 1
$BUN r3-validate.ts --file "$G/🔣️.json" | tail -4 | cut -c1-400
$BUN r3-validate.ts --file "$G/🔣️.json" | tail -1 | grep -q "^VALID" || exit 1
$BUN r3-wheel-loader-suit.ts "$G/🔣️.json" | grep -c "authored violations 0" | grep -q "^4$" || { echo "print suitability violated"; exit 1; }
python3 r3-wheel-loader-write.py && python3 volumes-insert.py volumes/machinery/wheel-loader.json "$A" | tail -1
[ "$1" = "nocheck" ] && exit 0
for s in ${@:-50 100 200 500}; do
  $BUN volumes-check.ts machinery/wheel-loader --product --scale "$s" > "$G/check.s$s.txt" 2>&1
  python3 r3-wheel-loader-fails.py "$G/check.s$s.txt"
done
