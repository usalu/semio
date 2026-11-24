#!/bin/zsh
# 🧪️ Tries the work ops of a person on a copy of the state before this round (🗑️generated/silhouette/R2-HA/work/<slug>/🔣️.json): every silhouette check of every scale, `validateAsset` and the three-views sweep of the copy beside the backup of before, and the product check CLI on the copy: try.sh <slug>   (QUICK=1: silhouette and validate only)
source "${0:A:h}/env.sh"
a="$1"; o="$GEN/ops/$a.json"; d="$GEN/work/$a"; mkdir -p "$d"
cd "$S/📦️packages/🟦️typescript"
timeout 300 "$B" "$W/apply.ts" "$a" --ops "$o" --in "$GEN/backup/$a.json" --out "$d/🔣️.json" || exit 1
timeout 900 "$B" "$W/measure.ts" "$d/🔣️.json" | grep -E "bbox|OK|failing"
N=${N:-8} timeout 900 "$B" "$W/validate.ts" "$d/🔣️.json" "$GEN/backup/$a.json"
[[ -n "$QUICK" ]] && exit 0
N=4 timeout 900 "$B" "$W/sweep.ts" "$d/🔣️.json" "$GEN/backup/$a.json"
(cd "$S" && timeout 900 "$B" "🔨️modules/🗂️catalog/🩺️check/🟦️.ts" --file "$d/🔣️.json" 2>&1 | grep -E "^(✓|✗)|UNSUITABLE|unsuitable" | cut -c1-400 | head -8)
