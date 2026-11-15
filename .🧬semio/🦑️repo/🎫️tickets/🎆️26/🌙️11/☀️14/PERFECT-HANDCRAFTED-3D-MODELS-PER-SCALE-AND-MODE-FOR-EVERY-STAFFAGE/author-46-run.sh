#!/bin/sh
# usage: author-46-run.sh <pig|sheep|fox|squirrel> <scale>
cd "$(dirname "$0")"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
A=$1; S=$2
case $A in pig) ASSET=animals/pig;; sheep) ASSET=animals/sheep;; fox) ASSET=animals/red-fox;; squirrel) ASSET=animals/red-squirrel;; esac
mkdir -p 🗑️generated/author-46
$BUN author-46-$A.ts --scale $S --write || exit 1
MAP=$($BUN author-46-$A.ts --scale $S --map)
$BUN kit-thermal.ts $ASSET --scale $S --map "$MAP" --write > 🗑️generated/author-46/$A.s$S.thermal.txt 2>&1
$BUN author-46-$A.ts --scale $S --extras
$BUN author-46-$A.ts --scale $S --structure
$BUN author-46-$A.ts --scale $S --decisions
tail -25 🗑️generated/author-46/$A.s$S.thermal.txt | head -16
