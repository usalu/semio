#!/bin/zsh
# 🩺️ The product check CLI of several assets inside ONE grant of 5000 MB (start it with qsh), one process per asset, one after the other: QMEM=5000 QTIME=<s> qsh in-check.sh <id>…   (logs in $GEN/proof/<id>-check.log)
[[ -n "$MH3_GRANT" ]] || { echo "in-check.sh runs only inside a grant: use qsh"; exit 2; }
export BATCH=MH-3
source "${0:A:h}/../MR/env.sh"
d="$GEN/proof"; mkdir -p "$d"
for a in "$@"; do
  (cd "$S" && timeout 1800 "$B" "🔨️modules/🗂️catalog/🩺️check/🟦️.ts" --asset "$a") > "$d/$a-check.log" 2>&1; echo "$a check exit $?: $(grep -m1 -E '^(✓|✗)' "$d/$a-check.log" | cut -c1-160)"
done
