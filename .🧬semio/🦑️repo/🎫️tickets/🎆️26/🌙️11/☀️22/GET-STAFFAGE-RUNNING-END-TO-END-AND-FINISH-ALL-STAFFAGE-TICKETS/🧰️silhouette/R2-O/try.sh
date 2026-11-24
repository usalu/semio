#!/bin/zsh
# 🧪️ Tries the repair ops of an object on a copy: applies `ops/<id>.json` to the catalog file into 🗑️generated/silhouette/R2-O/work/<id>.json, then the silhouette checks with `checkVolumes` and `validateAsset` of the copy: try.sh <id> [scales]
source "${0:A:h}/env.sh"
mkdir -p "$GEN/work"
cd "$S/📦️packages/🟦️typescript"
timeout 900 "$B" "$W/apply.ts" "$1" --out "$GEN/work/$1.json" || exit 1
timeout 1500 "$B" "$W/measure.ts" "$GEN/work/$1.json" --volumes ${2:+--scales=$2}
N=${N:-8} timeout 900 "$B" "$W/validate.ts" "$GEN/work/$1.json"
