#!/bin/bash
# usage: author-71-render.sh category/slug "scales" frontAz frontEl  -> 🗑️generated/author-71/<slug>/s<N>.all.png (sheet + front-left-top + rear-right-low + straight front)
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
T="$(cd "$(dirname "$0")" && pwd)"; cd "$T"
a=$1; slug=${a#*/}; cat=${a%/*}; scales=${2:-50 100 200 500}; faz=${3:-90}; fel=${4:-5}
out="$T/🗑️generated/author-71/$slug"; mkdir -p "$out"
R="$T/🗑️generated/volumes-tools/render/$cat"
for s in $scales; do
  $BUN volumes-render.ts $a --scale $s --no-numbers >/dev/null 2>&1 && cp "$R/$slug.s$s.png" "$out/s$s.sheet.png"
  $BUN volumes-render.ts $a --scale $s --no-numbers --az -55 --el 28 >/dev/null 2>&1 && cp "$R/$slug.s$s.png" "$out/s$s.flt.png"
  $BUN volumes-render.ts $a --scale $s --no-numbers --az 215 --el -12 >/dev/null 2>&1 && cp "$R/$slug.s$s.png" "$out/s$s.rrl.png"
  $BUN volumes-render.ts $a --scale $s --no-numbers --az $faz --el $fel >/dev/null 2>&1 && cp "$R/$slug.s$s.png" "$out/s$s.front.png"
  $BUN author-71-stitch.ts "$out/s$s.all.png" "$out/s$s.sheet.png" "$out/s$s.flt.png" "$out/s$s.rrl.png" "$out/s$s.front.png" && echo "$out/s$s.all.png"
done
