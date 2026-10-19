#!/bin/zsh
# usage: tsflow.sh <plan.ts> <tag> <source-rel>   (runs dry importers first so last-importers.txt is fresh)
S=/private/tmp/claude-501/-Users-ueli-Documents-semio/214979e9-1d0c-472f-865a-73fad9fbbf95/scratchpad
G=../../🗑️generated
cd "$(dirname "$0")"
plan=$1; tag=$2; src=$3
bun run-split.ts $PWD/$plan --importers > $G/c7w-$tag-dry.log 2>&1
(echo "$src"; cat $S/last-importers.txt) > $G/c7w-$tag-base-list.txt
cp "/Users/ueli/Documents/semio/$src" $PWD/orig-$tag.ts
bun tscheck.ts $G/c7w-$tag-base-list.txt $G/c7w-$tag-base-diags.tsv
bun run-split.ts $PWD/$plan --write > $G/c7w-$tag-write.log 2>&1
tail -4 $G/c7w-$tag-write.log
(cat $S/last-new-files.txt; cat $S/last-importers.txt) | sort -u > $G/c7w-$tag-after-list.txt
bun tscheck.ts $G/c7w-$tag-after-list.txt $G/c7w-$tag-after-diags.tsv
bun lossless.ts $PWD/orig-$tag.ts $S/last-new-files.txt 2>&1 | head -6
