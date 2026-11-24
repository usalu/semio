#!/bin/zsh
# 🚢️ Applies the repair of one asset to its catalog file (twice, the second run must find it in place), renders the 1:50 and 1:500 sheets and runs the proof: ship.sh <id>
source "${0:A:h}/env.sh"
id="$1"
cd "$W" && timeout 120 "$B" "fix-$id.ts" && timeout 120 "$B" "fix-$id.ts" || exit 1
"$W/look.sh" after "$CAT/$id/🔣️.json" "$id" 50 500 | sed 's/.*look/look/'
"$W/prove.sh" "$id"
