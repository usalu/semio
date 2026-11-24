#!/bin/zsh
# 🩺️ Proof of people outside the test runner, one process after the other: `validateAsset` issue count, the three-views sweep count and the verdict line of the product check CLI, each beside the same measurement of the backup taken before the repair; logs in 🗑️generated/silhouette/SH-2/proof. proof.sh <slug>…
source "${0:A:h}/env.sh"
mkdir -p "$GEN/proof"
for a in "$@"; do
  (cd "$S/📦️packages/🟦️typescript" && N=12 timeout 600 "$B" "$W/validate.ts" "$a" "$GEN/backup/$a.json") > "$GEN/proof/$a-validate.log" 2>&1
  (cd "$S/📦️packages/🟦️typescript" && N=4 timeout 900 "$B" "$W/sweep.ts" "$a" "$GEN/backup/$a.json") > "$GEN/proof/$a-sweep.log" 2>&1
  (cd "$S" && timeout 900 "$B" "🔨️modules/🗂️catalog/🩺️check/🟦️.ts" --asset "$a") > "$GEN/proof/$a-check.log" 2>&1
  echo "$a | validate $(grep -E "^$a\s" "$GEN/proof/$a-validate.log" | cut -f2 | tr '\n' '/' ) | sweep $(grep -E "findings" "$GEN/proof/$a-sweep.log" | sed -E 's/^[^:]*: ([0-9]+) findings.*/\1/' | tr '\n' '/') | $(grep -m1 -E '^(✓|✗)' "$GEN/proof/$a-check.log" | cut -c1-120)"
done
