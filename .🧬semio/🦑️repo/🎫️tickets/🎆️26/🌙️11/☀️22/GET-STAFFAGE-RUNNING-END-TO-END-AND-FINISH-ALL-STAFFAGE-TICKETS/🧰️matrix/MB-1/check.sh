#!/bin/zsh
# ✅️ The product check CLI of assets, one process each, logs in 🗑️generated/matrix/MB-1/check: check.sh <id>…
source "${0:A:h}/env.sh"
mkdir -p "$GEN/check"
for id in "$@"; do
  (cd "$S" && timeout 1800 "$B" "🔨️modules/🗂️catalog/🩺️check/🟦️.ts" --asset "$id") > "$GEN/check/$id.log" 2>&1
  echo "$id check exit $?: $(grep -m2 -E '^(✓|✗)' "$GEN/check/$id.log" | cut -c1-150 | tr '\n' ' ')"
done
