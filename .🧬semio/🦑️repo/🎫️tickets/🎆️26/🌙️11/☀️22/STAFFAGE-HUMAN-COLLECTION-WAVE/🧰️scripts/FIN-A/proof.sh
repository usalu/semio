#!/bin/zsh
# 🖼️ Default-pose proof of an asset: three-view sheets at 1:50, 1:100, 1:200 and 1:500 joined in one PNG under 🗑️generated/FIN-A/proof: proof.sh <slug|file.json> [name]
source "${0:A:h}/env.sh"
n=${2:-${1:t:r}}; rm -rf "$GEN/sheets/$n"; mkdir -p "$GEN/proof"
DPI=${DPI:-100} "$W/sheet.sh" "$1" "$n" "50 100 200 500" > /dev/null
find "$GEN/sheets/$n" -name "*.svg" -delete
(cd "$W" && $B montage.ts "$GEN/sheets/$n" "$GEN/proof/$n.png" ${HEIGHT:-1100})
