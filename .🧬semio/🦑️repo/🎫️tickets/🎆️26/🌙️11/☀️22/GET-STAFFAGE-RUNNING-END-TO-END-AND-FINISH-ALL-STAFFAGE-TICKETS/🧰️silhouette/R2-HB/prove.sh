#!/bin/zsh
# 🩺️ Proof of people, one process after the other: the real `🧊️catalog-silhouette` test, `validateAsset` (now / backup of before this round), the three-views sweep (now / before), the product check CLI, then the suites `🧊️three-views`, `🌡️catalog-physics`, `🫁️breathing-catalog`, `✅️asset-validation`, `⚡️asset-power` with the name filter; logs in 🗑️generated/silhouette/R2-HB/proof and /vt. prove.sh <slug>…   (QUICK=1 leaves out the five other suites)
source "${0:A:h}/env.sh"
mkdir -p "$GEN/proof"
for a in "$@"; do
  echo "$a | $("$W/vt.sh" "sil-$a" 🧊️catalog-silhouette "$a")"
  (cd "$S/📦️packages/🟦️typescript" && N=12 timeout 900 "$B" "$W/validate.ts" "$a" "$GEN/backup/$a.json") > "$GEN/proof/$a-validate.log" 2>&1
  (cd "$S/📦️packages/🟦️typescript" && N=4 timeout 1500 "$B" "$W/sweep.ts" "$a" "$GEN/backup/$a.json") > "$GEN/proof/$a-sweep.log" 2>&1
  (cd "$S" && timeout 1500 "$B" "🔨️modules/🗂️catalog/🩺️check/🟦️.ts" --asset "$a") > "$GEN/proof/$a-check.log" 2>&1
  echo "$a | validateAsset $(grep -E "^$a\s" "$GEN/proof/$a-validate.log" | cut -f2 | tr '\n' '/') | sweep $(grep -E "findings" "$GEN/proof/$a-sweep.log" | sed -E 's/^[^:]*: ([0-9]+) findings.*/\1/' | tr '\n' '/') | check $(grep -m1 -E '^(✓|✗)' "$GEN/proof/$a-check.log" | cut -c1-160)"
  [[ -n "$QUICK" ]] && continue
  "$W/others.sh" "$a" "$a" | sed "s/^/$a | /"
done
