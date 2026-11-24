#!/bin/zsh
# 🖼️ The guard's raster (tile) around a point as PNG with the worst places of the view: w.sh <id> <scale> <view> <x> <y> <radius> [ops.json|-] [count]
source "${0:A:h}/env.sh"
mkdir -p "$GEN/png"
OUT="$GEN/png/$1-$2-$3-$4-$5.png"
cd "$S/📦️packages/🟦️typescript" && timeout 1500 "$B" "$W/where.ts" "$1" "$2" "$3" "${7:--}" "${8:-6}" "$OUT" "$4" "$5" "$6" | cut -c1-420
echo "$OUT"
