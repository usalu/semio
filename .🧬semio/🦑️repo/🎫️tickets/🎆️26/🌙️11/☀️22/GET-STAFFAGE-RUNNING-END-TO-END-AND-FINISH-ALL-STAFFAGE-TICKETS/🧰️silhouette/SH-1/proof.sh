#!/bin/zsh
# 🩺️ Proof of people, one process after the other: every silhouette check (measure.ts), `validateAsset` issue count and the verdict line of the staffage check CLI; logs in 🗑️generated/silhouette/SH-1/proof. proof.sh <slug>…
source "${0:A:h}/env.sh"
mkdir -p "$GEN/proof"
for a in "$@"; do
  (cd "$W" && timeout 600 "$B" measure.ts "$a") > "$GEN/proof/$a-measure.log" 2>&1
  (cd "$W" && N=12 timeout 600 "$B" validate.ts "$a") > "$GEN/proof/$a-validate.log" 2>&1
  (cd "$S" && timeout 900 "$B" "🔨️modules/🗂️catalog/🩺️check/🟦️.ts" --asset "$a") > "$GEN/proof/$a-check.log" 2>&1
  echo "$a | $(tail -1 "$GEN/proof/$a-measure.log") | $(head -1 "$GEN/proof/$a-validate.log" | cut -f2) | $(grep -m1 -E '^(✓|✗)' "$GEN/proof/$a-check.log" | cut -c1-160)"
done
