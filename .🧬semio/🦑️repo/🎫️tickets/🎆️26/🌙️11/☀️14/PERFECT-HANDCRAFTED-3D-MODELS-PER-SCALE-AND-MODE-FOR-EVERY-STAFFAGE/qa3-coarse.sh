#!/bin/bash
# usage: qa3-coarse.sh slug -> qa3-<slug>.coarse.png : s200 + s500 rows of front-left-top, rear-right-low and straight front (crop of the axonometric rows)
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1); T="$(cd "$(dirname "$0")" && pwd)"; d="$T/🗑️generated/qa3/$1"; cd "$T"
a=(); for s in 100 200 500; do [ -f "$d/qa3-$1.s$s.all.png" ] && a+=("$d/qa3-$1.s$s.all.png@756@1182" "$d/qa3-$1.s$s.all.png@1608@2034"); done
$BUN qa3-coarse.ts "$d/qa3-$1.coarse.png" "${a[@]}"
