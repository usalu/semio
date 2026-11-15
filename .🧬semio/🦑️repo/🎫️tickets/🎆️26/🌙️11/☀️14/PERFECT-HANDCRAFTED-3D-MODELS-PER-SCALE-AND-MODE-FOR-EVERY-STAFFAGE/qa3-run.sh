#!/bin/bash
# usage: qa3-run.sh category/slug [scales...] -> 🗑️generated/qa3/<slug>/qa3-<slug>.s<N>.all.png = sheet + front-left-top + rear-right-low + straight front
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
T="$(cd "$(dirname "$0")" && pwd)"; cd "$T"
a=$1; shift; slug=${a#*/}; cat=${a%/*}
out="$T/🗑️generated/qa3/$slug"; mkdir -p "$out"
R="$T/🗑️generated/volumes-tools/render/$cat"
scales=${@:-50 100 200 500}
for s in $scales; do
  $BUN volumes-render.ts $a --scale $s --no-numbers >/dev/null 2>&1 || continue
  cp "$R/$slug.s$s.png" "$out/qa3-$slug.s$s.sheet.png"
  $BUN volumes-render.ts $a --scale $s --no-numbers --az -55 --el 28 >/dev/null 2>&1 && cp "$R/$slug.s$s.png" "$out/qa3-$slug.s$s.flt.png"
  $BUN volumes-render.ts $a --scale $s --no-numbers --az 215 --el -12 >/dev/null 2>&1 && cp "$R/$slug.s$s.png" "$out/qa3-$slug.s$s.rrl.png"
  $BUN volumes-render.ts $a --scale $s --no-numbers --az ${FRONTAZ:-0} --el 4 >/dev/null 2>&1 && cp "$R/$slug.s$s.png" "$out/qa3-$slug.s$s.front.png"
  $BUN qa3-stitch.ts "$out/qa3-$slug.s$s.all.png" "$out/qa3-$slug.s$s.sheet.png" "$out/qa3-$slug.s$s.flt.png" "$out/qa3-$slug.s$s.rrl.png" "$out/qa3-$slug.s$s.front.png" 2>/dev/null
done
ls "$out" | grep all
