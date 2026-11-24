#!/bin/zsh
# 🖼️ Several raster pictures of one view in one go: pngs.sh <id|file> <scale> <view> <ops|-> <name:x:y:r>…   (PNG files in 🗑️generated/silhouette/R3-M/png)
source "${0:A:h}/env.sh"; cd "$W"; mkdir -p "$GEN/png"
id="$1"; s="$2"; v="$3"; ops="$4"; shift 4
for q in "$@"; do p=("${(@s/:/)q}"); timeout 300 "$B" png.ts "$id" "$s" "$v" "$GEN/png/$p[1].png" "$ops" "$p[2]" "$p[3]" "$p[4]" | sed 's/.*png\///'; done
