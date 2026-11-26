#!/bin/zsh
# 🩺️ The check CLI of catalog assets, one after the other inside the grant of its caller: check-asset.sh <id>… writes `$GEN/before/<id>-check.log`.
source "${0:A:h}/env.sh"
mkdir -p "$GEN/before"
for a in "$@"; do (cd "$S" && timeout 1200 "$B" "🔨️modules/🗂️catalog/🩺️check/🟦️.ts" --asset "$a") > "$GEN/before/$a-check.log" 2>&1; echo "$a check exit $?"; done
