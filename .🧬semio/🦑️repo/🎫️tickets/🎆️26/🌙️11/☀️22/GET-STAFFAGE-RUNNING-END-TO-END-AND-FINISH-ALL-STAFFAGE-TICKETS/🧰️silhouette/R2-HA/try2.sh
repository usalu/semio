#!/bin/zsh
# 🧪️ The second half of a trial of the work copy of a person (after `QUICK=1 try.sh`): the three-views sweep of the copy beside the backup, and the product check CLI on the copy with its print verdicts: try2.sh <slug>
source "${0:A:h}/env.sh"
a="$1"; d="$GEN/work/$a"
(cd "$S/📦️packages/🟦️typescript" && N=4 timeout 160 "$B" "$W/sweep.ts" "$d/🔣️.json" "$GEN/backup/$a.json")
(cd "$S" && timeout 160 "$B" "🔨️modules/🗂️catalog/🩺️check/🟦️.ts" --file "$d/🔣️.json" 2>&1 | grep -E "^(✓|✗)|UNSUITABLE|unsuitable|✗" | cut -c1-400 | head -8)
