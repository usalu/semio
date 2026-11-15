#!/bin/sh
# usage: author-54-pipe.sh <slug-gen-prefix> <asset> <scale> [fixscript]
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
GEN=$1; ASSET=$2; S=$3; FIX=$4
$BUN $GEN $S >/dev/null || exit 1
$BUN kit-thermal.ts $ASSET --scale $S --write 2>&1 | grep -E "^[a-z-]+ +[0-9]+ +[0-9.]+ +|C15|written" | cut -c1-200
$BUN kit-structure.ts $ASSET --scale $S --write 2>&1 | grep -E "outside|written" | cut -c1-200
[ -n "$FIX" ] && $BUN $FIX $S
