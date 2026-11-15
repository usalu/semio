#!/bin/bash
# usage: qa2-mont.sh slug -> qa2-<slug>.A.png (az -55 four scales) and .B.png (az 215 four scales)
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1); T="$(cd "$(dirname "$0")" && pwd)"; d="$T/🗑️generated/qa2/$1"; cd "$T"
f=(); g=(); for s in 50 100 200 500; do [ -f "$d/qa2-$1.s$s.flt.png" ] && f+=("$d/qa2-$1.s$s.flt.png") && g+=("$d/qa2-$1.s$s.rrl.png"); done
$BUN qa2-montage.ts "$d/qa2-$1.A.png" "${f[@]}"; $BUN qa2-montage.ts "$d/qa2-$1.B.png" "${g[@]}"
