#!/bin/zsh
# 🩺️ The product check CLI per asset (`S/🔨️modules/🗂️catalog/🩺️check/🟦️.ts --asset <id>`), one process after the other, log in $GEN/log/check-<id>.txt and one line per asset with its first verdict line: check.sh [<id>…]   (run it through q.sh with 1000 MB or inside a larger grant)
source "${0:h}/env.sh"
ids=("$@"); [[ ${#ids} -eq 0 ]] && ids=($IDS)
for a in $ids; do
  (cd "$S" && timeout 1800 "$B" "🔨️modules/🗂️catalog/🩺️check/🟦️.ts" --asset "$a") > "$GEN/log/check-$a.txt" 2>&1
  echo "$a | check exit $? | $(grep -m1 -E '^(✓|✗)' "$GEN/log/check-$a.txt" | cut -c1-100)"
done
