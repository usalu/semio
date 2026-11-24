#!/bin/zsh
# 🧭️ Writes `ops/<id>.json` from the first gate log of the asset (`$GEN/gate/<id>.0.log`): the proposals of `suggest.ts` for the bones the gate reports: propose.sh <id> [suggest flags…]
BATCH=MA source "${0:A:h}/../MR/env.sh"
a="$1"; shift
"$B" "$W/suggest.ts" "$GEN/backup/$a.json" --pairs "$(python3 "$W/reported.py" "$GEN/gate/$a.0.log")" "$@" > "$W/ops/$a.json"
echo "$a: $(grep -c '"op"' "$W/ops/$a.json") ops"
