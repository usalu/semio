#!/bin/zsh
# 🧫️ The body search of one plant on a copy, inside the grant of the worker: `trial.sh <id> [flags of ../MV-3/body.ts]` runs `body.ts` on `$GEN/backup/<id>.json` (or on `$GEN/base/<id>.json` when a step before it made one), rounds the ends of the bodies (`caps.ts`, flags in CAPS) into `body/<id>.json`, applies them to the copy as `$GEN/try/<id>.json`, then prints the gate over every subject (`../MR/gate.ts --soft`) and what the bodies show beside the outlines in the wind poses (`../MV-3/visible.ts --prefix mvb-`).
source "${0:A:h}/env.sh"
a="$1"; shift; mkdir -p "$W/body" "$GEN/try" "$GEN/log"; src="$GEN/backup/$a.json"; [[ -f "$GEN/base/$a.json" ]] && src="$GEN/base/$a.json"
cd "$T"
timeout 3000 "$B" "$U/🧰️matrix/MV-3/body.ts" "$src" --save "$GEN/try/$a-raw-ops.json" "$@" > "$GEN/log/$a-body.log" 2>&1; echo "body exit $? $(grep -c '^left' "$GEN/log/$a-body.log") left; $(tail -2 "$GEN/log/$a-body.log" | head -1 | cut -c1-200)"
timeout 300 "$B" "$W/caps.ts" "$GEN/try/$a-raw-ops.json" "$W/body/$a.json" ${=CAPS} | tail -1
timeout 3000 "$B" "$U/🧰️matrix/MV-3/body.ts" "$src" --ops "$W/body/$a.json" --out "$GEN/try/$a.json" 2>&1 | tail -2 | cut -c1-200
timeout 3000 "$B" "$M/gate.ts" "$GEN/try/$a.json" --soft > "$GEN/log/$a-try-gate.log" 2>&1; echo "gate exit $? $(tail -1 "$GEN/log/$a-try-gate.log")"
timeout 3000 "$B" "$U/🧰️matrix/MV-3/visible.ts" "$GEN/try/$a.json" --prefix mvb- > "$GEN/log/$a-try-visible.log" 2>&1; echo "visible exit $? $(tail -1 "$GEN/log/$a-try-visible.log" | cut -c1-260)"
