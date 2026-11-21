#!/bin/bash
# 🎞️ Render every r5 sheet of one asset (body, structure, thermal cut per scale; poses at 1:50; 2D per scale) into 🗑️generated/r5-<slug>/<tag>/.
cd "$(dirname "$0")"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
A=$1; TAG=$2; shift 2
SLUG=${A#*/}
OUT="🗑️generated/r5-$SLUG/$TAG"; mkdir -p "$OUT"
SH="🗑️generated/r3-sheet/humans"
for S in 50 100 200 500; do
  $BUN r3-sheet.ts $A --scale $S --no-labels >/dev/null 2>"$OUT/err.s$S.body.txt" && cp "$SH/$SLUG.s$S.body.png" "$OUT/s$S.body.png"
  $BUN r3-sheet.ts $A --scale $S --list structure --no-labels >/dev/null 2>"$OUT/err.s$S.st.txt" && cp "$SH/$SLUG.s$S.structure.png" "$OUT/s$S.structure.png"
  $BUN r3-sheet.ts $A --scale $S --list thermal --cut --no-labels >/dev/null 2>"$OUT/err.s$S.th.txt" && cp "$SH/$SLUG.s$S.thermal.cut.png" "$OUT/s$S.thermal.cut.png"
done
for P in "$@"; do
  $BUN r3-sheet.ts $A --scale 50 --pose $P --no-labels >/dev/null 2>"$OUT/err.pose.$P.txt" && cp "$SH/$SLUG.s50.body.$P.png" "$OUT/s50.pose.$P.png"
done
ls "$OUT"
