#!/bin/zsh
# 🖼️ Writes the mask picture of one view (drawn-only red, volume-only blue, both grey) to 🗑️generated/silhouette/SC/png/<name>.png: p.sh <name> <id> <scale> <view> [x y radius]
source "${0:A:h}/env.sh"
mkdir -p "$GEN/png"; n="$1"; shift
cd "$S/📦️packages/🟦️typescript" && timeout 600 "$B" "$W/png.ts" "$1" "$2" "$3" "$GEN/png/$n.png" - "${@:4}" 2>&1 | cut -c1-300
