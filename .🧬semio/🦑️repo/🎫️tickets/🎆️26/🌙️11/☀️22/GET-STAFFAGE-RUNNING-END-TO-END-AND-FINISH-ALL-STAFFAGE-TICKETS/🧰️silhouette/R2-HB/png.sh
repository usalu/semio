#!/bin/zsh
# 🖼️ Overlay PNG of the drawn and the volume mask of one view as the guard rasterises them (drawn-only red, volume-only blue, both grey), whole view or a window, optional trial ops first: png.sh <slug|file> <scale> <view> <ops.json|-> [x y radius]; prints the file in 🗑️generated/silhouette/R2-HB/png.
source "${0:A:h}/env.sh"; mkdir -p "$GEN/png"
out="$GEN/png/${1:t:r}.s$2.$3${5:+.$5_$6}.png"
cd "$S/📦️packages/🟦️typescript" && timeout 600 "$B" "$W/png.ts" "$1" "$2" "$3" "$out" "$([[ -n "$4" && "$4" != - && "$4" != /* ]] && echo "$W/$4" || echo "${4:--}")" "${@:5}"
