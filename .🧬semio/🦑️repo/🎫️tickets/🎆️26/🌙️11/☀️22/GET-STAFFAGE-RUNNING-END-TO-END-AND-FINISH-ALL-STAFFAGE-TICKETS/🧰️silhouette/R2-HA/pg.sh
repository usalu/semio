#!/bin/zsh
# 🖼️ Overlay PNG of a window of one view with the work ops applied, written to 🗑️generated/silhouette/R2-HA/png/<name>.png: pg.sh <name> <slug> <scale> <view> <x> <y> <radius>
source "${0:A:h}/env.sh"
n="$1"; a="$2"; shift 2; mkdir -p "$GEN/png"; o="$GEN/ops/$a.json"; [[ -f "$o" ]] || o="-"
cd "$S/📦️packages/🟦️typescript" && timeout 300 "$B" "$W/png.ts" "$a" "$1" "$2" "$GEN/png/$n.png" "$o" "$3" "$4" "$5" | sed 's/.*png\///'
