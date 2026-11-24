#!/bin/zsh
# 🔍️ zoom.ts into 🗑️generated/matrix/MB-1/zoom/<id>.<scale>.png: zoom.sh <id> <scale> [pose|-] [red ids] [views]
source "${0:A:h}/env.sh"
mkdir -p "$GEN/zoom"
cd "$T" && timeout 300 "$B" "$W/zoom.ts" "$1" "$2" "$GEN/zoom/$1.$2.png" "${3:--}" "${4:-}" ${5:+"$5"} 2>&1 | tail -1
