#!/bin/zsh
set -e
cd "${0:h}"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun)
slug=$1
OUT=🗑️generated/authoring-animals
A=$(ls -d "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/"*animals/$slug)/🔣️.json
$BUN authoring-animals/$slug.ts
$BUN volume-check.ts $slug --volume volumes/animals/$slug.json --json > $OUT/$slug.check.json || true
python3 volume-insert.py volumes/animals/$slug.json "$A"
$BUN volume-check.ts $slug | grep -E "x \(|y \(|z \(|IoU|PASS|FAIL" | cut -c1-120
$BUN authoring-animals-validate.ts $slug > $OUT/$slug.validate.txt 2>&1 || true
head -4 $OUT/$slug.validate.txt | cut -c1-200
grep -c "components 1" $OUT/$slug.validate.txt || true
grep -E "components [2-9]" $OUT/$slug.validate.txt | cut -c1-160 || true
