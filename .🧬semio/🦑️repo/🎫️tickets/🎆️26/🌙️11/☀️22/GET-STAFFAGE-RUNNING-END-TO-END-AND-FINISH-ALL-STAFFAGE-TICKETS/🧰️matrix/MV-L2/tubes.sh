#!/bin/zsh
# 🧪️ The tube search of one plant on a copy, inside the grant of the worker: `tubes.sh <id> [flags of tube.ts]` runs `tube.ts` on `$GEN/backup/<id>.json`, saves the lofts as `tube/<id>.json`, writes `$GEN/base/<id>.json` (the source of a following `trial.sh`) and `$GEN/try/<id>.json`, then prints the gate over every subject, print suitability at the keys and both ends of every band and what plates are left to show.
source "${0:A:h}/env.sh"
a="$1"; shift; mkdir -p "$W/tube" "$GEN/try" "$GEN/base" "$GEN/log"
cd "$T"
timeout 3000 "$B" "$W/tube.ts" "$GEN/backup/$a.json" --save "$W/tube/$a.json" --out "$GEN/base/$a.json" "$@" > "$GEN/log/$a-tube.log" 2>&1; echo "tube exit $? $(grep -c '^left' "$GEN/log/$a-tube.log") left, $(grep -c '^ISSUE' "$GEN/log/$a-tube.log") issues; $(tail -2 "$GEN/log/$a-tube.log" | head -1 | cut -c1-200)"
cp "$GEN/base/$a.json" "$GEN/try/$a.json"
timeout 3000 "$B" "$M/gate.ts" "$GEN/try/$a.json" --soft > "$GEN/log/$a-try-gate.log" 2>&1; echo "gate exit $? $(tail -1 "$GEN/log/$a-try-gate.log")"
timeout 3000 "$B" "$U/🧰️matrix/MC/prints.ts" "$GEN/try/$a.json" 2>&1 | tail -4 | cut -c1-300
