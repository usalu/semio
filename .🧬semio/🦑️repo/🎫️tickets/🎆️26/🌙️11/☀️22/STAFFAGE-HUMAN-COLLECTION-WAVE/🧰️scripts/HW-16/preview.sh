#!/bin/sh
# 🖼️ Renders previews of an HW-16 asset: preview.sh <slug> <scale> <view> <viewpoint|views> [extra preview flags]
B=/Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
S="/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage"
OUT="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/STAFFAGE-HUMAN-COLLECTION-WAVE/🗑️generated/HW-16/$1"
mkdir -p "$OUT"
slug=$1; scale=$2; view=$3; vp=$4; shift 4
cd "$S"
if [ "$vp" = "views" ]; then
  $B "🔨️modules/🎨️depiction/🖨️preview/🟦️.ts" --asset "$slug" --scale "$scale" --view "$view" --views --projection third --flow none --no-png --out "$OUT" "$@"
else
  $B "🔨️modules/🎨️depiction/🖨️preview/🟦️.ts" --asset "$slug" --scale "$scale" --view "$view" --viewpoint "$vp" --flow none --no-png --out "$OUT" "$@"
fi
cd "$OUT" && for f in $(ls -t *.svg | head -1); do $B "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/STAFFAGE-HUMAN-COLLECTION-WAVE/🧰️scripts/HW-16/svg2png.ts" ${DPI:-150} "$OUT/$f"; done
