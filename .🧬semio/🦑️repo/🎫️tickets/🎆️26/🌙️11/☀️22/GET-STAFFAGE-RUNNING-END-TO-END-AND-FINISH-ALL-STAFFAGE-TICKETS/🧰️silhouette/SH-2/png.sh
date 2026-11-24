#!/bin/zsh
# 🖼️ Overlay PNG of the drawn and the volume mask of one view as the guard rasterises them (drawn-only red, volume-only blue, both grey), whole view or a window: png.sh <slug> <scale> <view> [x y radius]; prints the file in 🗑️generated/silhouette/SH-2/png.
source "${0:A:h}/env.sh"; mkdir -p "$GEN/png"
out="$GEN/png/$1.s$2.$3${4:+.$4_$5}.png"
cd "$S/📦️packages/🟦️typescript" && timeout 600 "$B" "$W/png.ts" "$1" "$2" "$3" "$out" - "${@:4}"
