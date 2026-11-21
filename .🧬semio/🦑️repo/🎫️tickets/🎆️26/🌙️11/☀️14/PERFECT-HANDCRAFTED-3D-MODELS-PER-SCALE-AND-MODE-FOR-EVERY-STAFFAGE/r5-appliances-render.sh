#!/bin/bash
# 🔌 Round 5 appliances (E03): render body, structure, thermal cut per scale, every pose and clip frame at 1:50 and the 2D per scale into 🗑️generated/r5-<slug>/<tag>/. Usage: r5-vehicles-render.sh <category/slug> <tag> <what: s2p> [pose ... | clip@t ...]
cd "$(dirname "$0")"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
A=$1; TAG=$2; WHAT=$3; shift 3
SLUG=${A#*/}; CAT=${A%/*}
OUT="🗑️generated/r5-$SLUG/$TAG"; mkdir -p "$OUT"
SH="🗑️generated/r3-sheet/$CAT"
SCALES=${SCALES:-"50 100 200 500"}
if [[ "$WHAT" == *s* ]]; then
for S in $SCALES; do
  $BUN r3-sheet.ts $A --scale $S --no-labels >/dev/null 2>>"$OUT/err.txt" && cp "$SH/$SLUG.s$S.body.png" "$OUT/s$S.body.png"
  $BUN r3-sheet.ts $A --scale $S --list structure --no-labels >/dev/null 2>>"$OUT/err.txt" && cp "$SH/$SLUG.s$S.structure.png" "$OUT/s$S.structure.png"
  $BUN r3-sheet.ts $A --scale $S --list thermal --cut --no-labels >/dev/null 2>>"$OUT/err.txt" && cp "$SH/$SLUG.s$S.thermal.cut.png" "$OUT/s$S.thermal.cut.png"
done
fi
if [[ "$WHAT" == *2* ]]; then
for S in $SCALES; do
  $BUN r3-2d.ts $A --scale $S >/dev/null 2>>"$OUT/err.txt" && cp "🗑️generated/r3-2d/$CAT/$SLUG.s$S.png" "$OUT/2d.s$S.png"
done
fi
for P in "$@"; do
  case $P in
    *@*) C=${P%@*}; T=${P#*@}; $BUN r3-sheet.ts $A --scale 50 --clip $C --t $T --no-labels >/dev/null 2>>"$OUT/err.txt" && cp "$SH/$SLUG.s50.body.$C-t$T.png" "$OUT/s50.clip.$C-t$T.png";;
    *) $BUN r3-sheet.ts $A --scale 50 --pose $P --no-labels >/dev/null 2>>"$OUT/err.txt" && cp "$SH/$SLUG.s50.body.$P.png" "$OUT/s50.pose.$P.png";;
  esac
done
grep -i "not drawn\|error" "$OUT/err.txt" | head
ls "$OUT"
