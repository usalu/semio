#!/bin/bash
# usage: qa2-big.sh cat/slug scale list az el  -> qa2-<slug>.s<scale>.<list>.<az>.png in the asset folder
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1); T="$(cd "$(dirname "$0")" && pwd)"; cd "$T"
a=$1; slug=${a#*/}; cat=${a%/*}; out="$T/🗑️generated/qa2/$slug"; mkdir -p "$out"
$BUN volumes-render.ts $a --scale $2 --list $3 --az $4 --el $5 --no-numbers --width 1300 >/dev/null 2>&1 && cp "$T/🗑️generated/volumes-tools/render/$cat/$slug.s$2.$3.png" "$out/qa2-$slug.s$2.$3.$4.png" && echo "$out/qa2-$slug.s$2.$3.$4.png"
