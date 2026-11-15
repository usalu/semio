#!/bin/bash
# usage: qa2-run.sh category/slug [scales...]  -> renders sheets + 2 extra directions per scale into 🗑️generated/qa2/<slug>/
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
T="$(cd "$(dirname "$0")" && pwd)"; cd "$T"
a=$1; shift; slug=${a#*/}; cat=${a%/*}
out="$T/🗑️generated/qa2/$slug"; mkdir -p "$out"
R="$T/🗑️generated/volumes-tools/render/$cat"
scales=${@:-50 100 200 500}
for s in $scales; do
  $BUN volumes-render.ts $a --scale $s --no-numbers >/dev/null 2>&1 && cp "$R/$slug.s$s.png" "$out/qa2-$slug.s$s.sheet.png"
  $BUN volumes-render.ts $a --scale $s --no-numbers --az -55 --el 28 >/dev/null 2>&1 && cp "$R/$slug.s$s.png" "$out/qa2-$slug.s$s.flt.png"
  $BUN volumes-render.ts $a --scale $s --no-numbers --az 215 --el -12 >/dev/null 2>&1 && cp "$R/$slug.s$s.png" "$out/qa2-$slug.s$s.rrl.png"
done
ls "$out"
for s in $scales; do $BUN qa2-stitch.ts "$out/qa2-$slug.s$s.all.png" "$out/qa2-$slug.s$s.sheet.png" "$out/qa2-$slug.s$s.flt.png" "$out/qa2-$slug.s$s.rrl.png" 2>/dev/null; done
