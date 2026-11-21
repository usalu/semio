#!/bin/sh
# 🔎️ Round 5 furniture: downscaled copies of the sheets of one r5 tag folder for viewing. Usage: sh r5-furniture-shrink.sh <slug> <tag> [px]
cd "$(dirname "$0")/🗑️generated/r5-$1/$2" || exit 1
mkdir -p small
for f in *.png; do sips -Z "${3:-1400}" "$f" --out "small/$f" >/dev/null; done
ls small | wc -l
