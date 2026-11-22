#!/bin/sh
# 🖼️ Renders the three-view sheet of an asset pose at the given scales (light solid) and rasterises it: render.sh <slug> <out-subdir> [pose] [scales] [view]
B=/Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
H="$(cd "$(dirname "$0")" && pwd)"
S=/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage
G="$H/../../🗑️generated/HW-15/$2"
mkdir -p "$G"
P=""; [ -n "$3" ] && [ "$3" != "-" ] && P="--pose $3"
for s in ${4:-50 200}; do
  (cd "$S" && $B 🔨️modules/🎨️depiction/🖨️preview/🟦️.ts --asset "$1" ${CAT:+--catalog "$CAT"} --scale $s $P --view ${5:-light-solid} --views elevation,front,plan --projection third --flow none --out "$G" 2>&1 | grep -v "rsvg\|^ *at \|ToolMissing\|detail:\|tool:\|^$\|Bun v\|^[0-9]* |\|\^" )
done
$B "$H/png.ts" ${DPI:-200} "$G"/*.svg
rm -f "$G"/*.svg
ls "$G"
