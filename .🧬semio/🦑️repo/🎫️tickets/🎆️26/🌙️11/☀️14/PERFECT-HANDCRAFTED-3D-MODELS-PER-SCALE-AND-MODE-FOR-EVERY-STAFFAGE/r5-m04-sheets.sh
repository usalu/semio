#!/bin/sh
# 🖼️ Renders every r5 sheet of one M04 asset: body/structure/thermal-cut per scale, poses at 1:50, one clip frame, 2D per scale.
cd "$(dirname "$0")"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
A=$1; SCALES=$2; POSES=$3; CLIP=$4
for s in $SCALES; do
  $BUN r3-sheet.ts machinery/$A --scale $s >/dev/null
  $BUN r3-sheet.ts machinery/$A --scale $s --list structure >/dev/null
  $BUN r3-sheet.ts machinery/$A --scale $s --list thermal --cut >/dev/null
  $BUN r3-2d.ts machinery/$A --scale $s >/dev/null
done
for p in $POSES; do $BUN r3-sheet.ts machinery/$A --scale 50 --pose $p >/dev/null; done
[ -n "$CLIP" ] && $BUN r3-sheet.ts machinery/$A --scale 50 --clip ${CLIP%:*} --t ${CLIP#*:} >/dev/null
ls 🗑️generated/r3-sheet/machinery/$A.* 🗑️generated/r3-2d/machinery/$A.*
