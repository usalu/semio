#!/bin/zsh
# 🧪️ A trial of the ops of a person on a copy of its catalog file (🗑️generated/silhouette/R2-HB/trial/<slug>.json): every silhouette check, `validateAsset` and the three-views sweep of the copy; the catalog file stays as it is. trial.sh <slug> [ops.json] [scales]
source "${0:A:h}/env.sh"; mkdir -p "$GEN/trial"
a="$1"; f="$GEN/trial/$a.json"; cp "$CAT/$a/🔣️.json" "$f"
python3 "$W/apply.py" "$a" --ops "${2:-$W/ops/$a.json}" --file "$f"
cd "$S/📦️packages/🟦️typescript"
timeout 900 "$B" "$W/measure.ts" "$f" ${3:+--scales=$3} | grep -E "!|: (OK|[0-9]+ failing)"
N=6 timeout 900 "$B" "$W/validate.ts" "$f"
N=6 timeout 1500 "$B" "$W/sweep.ts" "$f"
