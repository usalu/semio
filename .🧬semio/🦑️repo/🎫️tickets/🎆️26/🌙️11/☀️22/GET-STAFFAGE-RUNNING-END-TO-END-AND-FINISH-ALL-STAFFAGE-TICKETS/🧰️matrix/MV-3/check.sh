#!/bin/zsh
# ✅️ The check CLI of the catalog for plants of batch MV-3, one process per plant, each through the memory queue (5000 MB): check.sh <id>…   (one line per plant appended to `$GEN/proof/<id>.txt`)
export BATCH=MV-3; source "${0:A:h}/../MR/env.sh"
d="$GEN/proof"; mkdir -p "$d"
for a in "$@"; do
  "$W/q.sh" ${CHECK_MB:-5000} "$S" 3000 "$B" "🔨️modules/🗂️catalog/🩺️check/🟦️.ts" --asset "$a" > "$d/$a-check.log" 2>&1; echo "check: exit $? $(grep -m1 -E '^(✓|✗)' "$d/$a-check.log" | cut -c1-160); $(grep -ciE 'prints|suitable' "$d/$a-check.log") suitability lines" | tee -a "$d/$a.txt"
done
