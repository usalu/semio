#!/bin/sh
# 🖼️ Renders the multiview sheet of one asset at the given scales (default pose unless a pose is given) and rasterises it with sharp.
# sh preview.sh <slug> <out-subdir> "<scales>" [pose] [view]
SLUG=$1; OUT=$2; SCALES=${3:-"50 100 200 500"}; POSE=$4; VIEW=${5:-light-solid}
R=/Users/ueli/Documents/semio
G="$R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/STAFFAGE-HUMAN-COLLECTION-WAVE/🗑️generated/HW-06/$OUT"
H="$R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/STAFFAGE-HUMAN-COLLECTION-WAVE/🧰️scripts/HW-06"
B=$R/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
mkdir -p "$G"
cd "$R/🎓️teaching/🏛️architecture/🧍️staffage"
for S in $SCALES; do
  if [ -n "$POSE" ]; then P="--pose $POSE"; else P=""; fi
  $B 🔨️modules/🎨️depiction/🖨️preview/🟦️.ts --asset $SLUG --scale $S $P --views elevation,front,plan --projection third --view $VIEW --no-png --out "$G" >/dev/null 2>"$G/err.txt" || tail -5 "$G/err.txt"
done
$B "$H/raster.ts" ${DPI:-300} "$G"/*.svg
