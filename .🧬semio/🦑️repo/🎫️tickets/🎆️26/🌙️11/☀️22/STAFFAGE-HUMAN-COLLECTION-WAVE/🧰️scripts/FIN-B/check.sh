#!/bin/zsh
# 🩺️ Runs the staffage check CLI one asset at a time and keeps one log per asset: check.sh <slug…>
source "${0:A:h}/env.sh"
mkdir -p "$GEN/check"
for a in "$@"; do
  (cd "$S" && $B "🔨️modules/🗂️catalog/🩺️check/🟦️.ts" --asset $a --samples 9 > "$GEN/check/$a.log" 2>&1); echo "$a exit $? $(grep -c '' "$GEN/check/$a.log") lines: $(grep -E '^[✓✗]' "$GEN/check/$a.log" | head -1 | cut -c1-200)"
done
echo CHECKS-DONE
