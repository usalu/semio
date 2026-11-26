#!/bin/zsh
# 🧵️ The loft search of MV-L1 for plants of round MV-L3 on copies, inside the grant of its caller: `loft-trial.sh <id>…` runs `../MV-L1/loft.ts` on `$GEN/backup/<id>.json`, saves the ops as `loft/<id>.json` and the copy as `$GEN/loft/<id>.json`, and prints the lines of the scales and every `ISSUE` or `LOST`.
source "${0:A:h}/env.sh"
mkdir -p "$W/loft" "$GEN/loft" "$GEN/log"; cd "$T"
for a in "$@"; do timeout 3000 "$B" "$U/🧰️matrix/MV-L1/loft.ts" "$GEN/backup/$a.json" --save "$W/loft/$a.json" --out "$GEN/loft/$a.json" > "$GEN/log/$a-loft.log" 2>&1; echo "$a loft exit $?"; grep -E '^1:|ISSUE|LOST' "$GEN/log/$a-loft.log" | cut -c1-260; tail -2 "$GEN/log/$a-loft.log" | cut -c1-200; done
