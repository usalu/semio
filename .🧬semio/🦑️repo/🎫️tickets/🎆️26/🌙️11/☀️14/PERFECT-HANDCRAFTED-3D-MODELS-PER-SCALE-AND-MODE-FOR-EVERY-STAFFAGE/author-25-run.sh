#!/bin/sh
# usage: author-25-run.sh <dog|cow|horse> <scale>  (body -> thermal -> structure for one scale)
cd "$(dirname "$0")"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
A=$1; S=$2
case $A in dog) ASSET=animals/labrador-dog;; cow) ASSET=animals/dairy-cow;; horse) ASSET=animals/horse;; esac
$BUN author-25-$A.ts --scale $S --write || exit 1
MAP=$($BUN author-25-$A.ts --scale $S --map)
$BUN kit-thermal.ts $ASSET --scale $S --map "$MAP" --write > 🗑️generated/author-25/$A.s$S.thermal.txt 2>&1
$BUN author-25-$A.ts --scale $S --extras
$BUN author-25-$A.ts --scale $S --structure
$BUN author-25-$A.ts --scale $S --decisions
tail -25 🗑️generated/author-25/$A.s$S.thermal.txt | head -14
