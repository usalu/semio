#!/bin/zsh
# 🔎️ probe.ts on the asset with ops applied (written to a work copy first): probe-ops.sh <id> <scale> <view> "x,y;…" [ops.json]
source "${0:A:h}/env.sh"; cd "$W"; mkdir -p "$GEN/work"
"$B" apply.ts "$1" --out "$GEN/work/$1.json" > /dev/null
[[ -f "$GEN/work/$1.json" ]] || cp "$(ls "$S/🗂️catalog"/*/"$1"/🔣️.json)" "$GEN/work/$1.json"
"$B" probe.ts "$GEN/work/$1.json" "$2" "$3" "$4"
