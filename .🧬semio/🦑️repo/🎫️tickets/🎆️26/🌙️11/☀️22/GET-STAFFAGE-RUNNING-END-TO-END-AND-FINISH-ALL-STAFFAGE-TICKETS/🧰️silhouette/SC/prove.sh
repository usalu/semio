#!/bin/zsh
# ✅️ The proof of one component, one process after the other, logs in 🗑️generated/silhouette/SC/proof/<id>: the gate measurement, the real 🧊️catalog-silhouette test, `validateAsset`, the product check CLI and the other suites: prove.sh <id>…
source "${0:A:h}/env.sh"
for id in "$@"; do
  d="$GEN/proof/$id"; mkdir -p "$d"
  cd "$S/📦️packages/🟦️typescript"
  timeout 900 "$B" "$W/measure.ts" "$id" > "$d/measure.log" 2>&1; echo "$id gate: $(tail -1 "$d/measure.log")"
  echo "$id $(SUFFIX='( |:|$)' "$W/vt.sh" "proof-$id-silhouette" "🧊️catalog-silhouette" "$id")"
  L=400 timeout 300 "$B" "$W/validate.ts" "$id" > "$d/validate.log" 2>&1; echo "$id validateAsset: $(head -1 "$d/validate.log" | cut -f2-)"
  timeout 900 "$B" "$S/🔨️modules/🗂️catalog/🩺️check/🟦️.ts" --asset "$id" > "$d/check.log" 2>&1; echo "$id check CLI exit $?: $(grep -E '^[✓✗]' "$d/check.log" | head -3 | cut -c1-200)"
  "$W/suites.sh" "proof-$id" "$id" | sed "s/^/$id /"
done
