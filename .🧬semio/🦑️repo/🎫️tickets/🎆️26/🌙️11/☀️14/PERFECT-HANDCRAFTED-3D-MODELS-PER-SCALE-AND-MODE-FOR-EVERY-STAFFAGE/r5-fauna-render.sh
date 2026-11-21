#!/bin/bash
# 🎞️ Render the r5 review sheets of one animal: body, structure, thermal cut per scale, poses and one clip frame at 1:50, 2D per scale, into 🗑️generated/r5-<slug>/<tag>/.
cd "$(dirname "$0")"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
A=$1; TAG=$2; WHAT=$3; shift 3
SLUG=${A#*/}
OUT="🗑️generated/r5-$SLUG/$TAG"; mkdir -p "$OUT"
SH="🗑️generated/r3-sheet/animals"
if [[ "$WHAT" == *s* ]]; then
for S in 50 100 200 500; do
  $BUN r3-sheet.ts $A --scale $S --no-labels >/dev/null 2>>"$OUT/err.txt" && cp "$SH/$SLUG.s$S.body.png" "$OUT/s$S.body.png"
  $BUN r3-sheet.ts $A --scale $S --list structure --no-labels >/dev/null 2>>"$OUT/err.txt" && cp "$SH/$SLUG.s$S.structure.png" "$OUT/s$S.structure.png"
  $BUN r3-sheet.ts $A --scale $S --list thermal --cut --no-labels >/dev/null 2>>"$OUT/err.txt" && cp "$SH/$SLUG.s$S.thermal.cut.png" "$OUT/s$S.thermal.cut.png"
done
fi
if [[ "$WHAT" == *2* ]]; then
for S in 50 100 200 500; do
  $BUN r3-2d.ts $A --scale $S >/dev/null 2>>"$OUT/err.txt" && cp "🗑️generated/r3-2d/animals/$SLUG.s$S.png" "$OUT/2d.s$S.png"
done
fi
for P in "$@"; do
  case $P in
    *@*) C=${P%@*}; T=${P#*@}; $BUN r3-sheet.ts $A --scale 50 --clip $C --t $T --no-labels >/dev/null 2>>"$OUT/err.txt" && cp "$SH/$SLUG.s50.body.$C-t$T.png" "$OUT/s50.clip.$C-t$T.png";;
    *) $BUN r3-sheet.ts $A --scale 50 --pose $P --no-labels >/dev/null 2>>"$OUT/err.txt" && cp "$SH/$SLUG.s50.body.$P.png" "$OUT/s50.pose.$P.png";;
  esac
done
ls "$OUT"
