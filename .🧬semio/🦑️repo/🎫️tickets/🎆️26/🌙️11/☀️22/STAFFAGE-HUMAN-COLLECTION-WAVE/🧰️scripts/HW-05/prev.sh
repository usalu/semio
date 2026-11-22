#!/bin/sh
# 🖼️ Renders one asset at one scale in the three viewpoints (light-solid unless VIEW=), rasterised and trimmed: sh prev.sh <slug> <scale> [pose]
S="/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage"
B=/Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
H="$(cd "$(dirname "$0")" && pwd)"
O="$H/../../🗑️generated/HW-05/prev/$1"
mkdir -p "$O"
P=""; [ -n "$3" ] && P="--pose $3"
for v in elevation front plan; do
  (cd "$S" && $B 🔨️modules/🎨️depiction/🖨️preview/🟦️.ts --asset "$1" --scale "$2" $P --viewpoint $v --view "${VIEW:-light-solid}" --no-png --out "$O" >/dev/null 2>&1)
done
cd "$H" && $B png.ts --dpi "${DPI:-900}" --trim $(ls -t "$O"/*.svg | head -3)
