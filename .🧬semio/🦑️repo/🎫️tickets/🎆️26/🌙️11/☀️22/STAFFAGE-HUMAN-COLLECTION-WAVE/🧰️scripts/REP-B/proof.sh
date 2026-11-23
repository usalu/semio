#!/bin/zsh
# 🩺️ Final proof of people, one after the other: `validateAsset` issue count and the verdict line of the staffage check CLI (`--samples 9`); logs in 🗑️generated/REP-B/proof: proof.sh <slug>…
source "${0:A:h}/env.sh"
mkdir -p "$GEN/proof"
for a in "$@"; do
  (cd "$K/🧰️scripts/FIN-B" && N=12 "$B" validate.ts "$a") > "$GEN/proof/$a-validate.log" 2>&1
  (cd "$S" && "$B" "🔨️modules/🗂️catalog/🩺️check/🟦️.ts" --asset "$a" --samples 9) > "$GEN/proof/$a-check.log" 2>&1
  echo "$a | $(head -1 "$GEN/proof/$a-validate.log" | cut -f2) | $(grep -m1 -E '^(✓|✗)' "$GEN/proof/$a-check.log" | cut -c1-160)"
done
