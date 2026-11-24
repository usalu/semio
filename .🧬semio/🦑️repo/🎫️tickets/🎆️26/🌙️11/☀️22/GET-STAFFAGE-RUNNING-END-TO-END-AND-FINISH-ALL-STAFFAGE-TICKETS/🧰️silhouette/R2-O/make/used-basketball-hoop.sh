#!/bin/zsh
# 🏀️ Writes the repair ops of `used-basketball-hoop` (ops/used-basketball-hoop.json) from the catalog file before the repair: shapes the generator drew as the box or hull of a repeated or concave solid are redrawn by the outline of the solid (outline.ts).
source "${0:A:h}/../env.sh"
A=used-basketball-hoop; O="$W/ops/$A.json"; rm -f "$O"
cd "$S/📦️packages/🟦️typescript" || exit 1
ol() { timeout 600 "$B" "$W/outline.ts" $A "$1" "$2" "$O" --base "$O" "${@:3}" || exit 1 }
ol 50 front "anchor-nut=anchor-nut" "anchor-nut-back=anchor-nut-back"
ol 50 plan "ring-gusset=ring-gusset"
ol 100 elevation "ring-bracket=ring-bracket"
