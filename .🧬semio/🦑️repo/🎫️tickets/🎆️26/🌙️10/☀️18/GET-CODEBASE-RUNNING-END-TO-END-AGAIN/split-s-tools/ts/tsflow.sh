#!/bin/zsh
# usage: tsflow.sh <plan.ts> <tag> <source-rel>   (runs dry importers first so last-importers.txt is fresh)
S=/Users/ueli/.cache/split-s/ts
G=../../🗑️generated
cd "$(dirname "$0")"
plan=$1; tag=$2; src=$3
bun run-split.ts $PWD/$plan --importers > $G/split-s-$tag-dry.log 2>&1
(echo "$src"; cat $S/last-importers.txt) > $G/split-s-$tag-base-list.txt
cp "/Users/ueli/Documents/semio/$src" $PWD/orig-$tag.ts
bun tscheck.ts $G/split-s-$tag-base-list.txt $G/split-s-$tag-base-diags.tsv
bun run-split.ts $PWD/$plan --write > $G/split-s-$tag-write.log 2>&1
tail -4 $G/split-s-$tag-write.log
(cat $S/last-new-files.txt; cat $S/last-importers.txt) | sort -u > $G/split-s-$tag-after-list.txt
bun tscheck.ts $G/split-s-$tag-after-list.txt $G/split-s-$tag-after-diags.tsv
bun lossless.ts $PWD/orig-$tag.ts $S/last-new-files.txt 2>&1 | head -6
