#!/bin/sh
# 🎞️ Renders the round-5 sheets of one of the G12 human assets (body, structure, thermal cut at every scale; the given poses at 1:50) into 🗑️generated/r5-<slug>/<tag>/. Usage: ./r5-courier-with-hand-truck-render.sh humans/<slug> <tag> [pose...]
cd "$(dirname "$0")"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
A=$1; TAG=$2; shift 2
SLUG=${A#*/}
OUT="🗑️generated/r5-$SLUG/$TAG"; mkdir -p "$OUT"
SH="🗑️generated/r3-sheet/humans"
for S in 50 100 200 500; do
  $BUN r3-sheet.ts "$A" --scale $S --no-labels >/dev/null 2>&1 && cp "$SH/$SLUG.s$S.body.png" "$OUT/s$S.body.png"
  $BUN r3-sheet.ts "$A" --scale $S --list structure --no-labels >/dev/null 2>&1 && cp "$SH/$SLUG.s$S.structure.png" "$OUT/s$S.structure.png"
  $BUN r3-sheet.ts "$A" --scale $S --list thermal --cut --no-labels >/dev/null 2>&1 && cp "$SH/$SLUG.s$S.thermal.cut.png" "$OUT/s$S.thermal.cut.png"
done
for P in "$@"; do
  $BUN r3-sheet.ts "$A" --scale 50 --pose "$P" --no-labels >/dev/null 2>&1 && cp "$SH/$SLUG.s50.body.$P.png" "$OUT/s50.pose.$P.png"
done
ls "$OUT" | wc -l
