#!/bin/sh
# usage: author-24-th.sh <slug> <scale>: thermal coverage lines only
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
$BUN volumes-check.ts humans/$1 --scale $2 --rules volumes-thermal-coverage 2>&1 | grep -o "areaDeviation[^ ]*\|exposed [^]]*" | paste - - | head -20
