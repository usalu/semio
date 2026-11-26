#!/bin/zsh
# 🧫️ The body search of one plant on a copy, inside the grant of the worker: `trial.sh <id> [flags of ../MV-3/body.ts]` runs `body.ts` on `$GEN/backup/<id>.json` (or on `$GEN/base/<id>.json` when a step before it made one), rounds the shoulders and ends of the bodies (`caps.ts`; its depth radius is DEEP, default 0.5, times the tolerance of the asset, further flags in CAPS, else in `flags/<id>.caps`) into `body/<id>.json`, applies them to the copy as `$GEN/try/<id>.json`, then prints what the bodies show beside the outlines in the default pose (`rest.ts`), the gate over every subject (`../MR/gate.ts --soft`) and what the bodies show beside the outlines in the wind poses (`../MV-3/visible.ts --prefix mvb-`).
source "${0:A:h}/env.sh"
a="$1"; shift; mkdir -p "$W/body" "$GEN/try" "$GEN/log"; src="$GEN/backup/$a.json"; [[ -f "$GEN/base/$a.json" ]] && src="$GEN/base/$a.json"
cd "$T"
timeout 3000 "$B" "$U/🧰️matrix/MV-3/body.ts" "$src" --save "$GEN/try/$a-raw-ops.json" "$@" > "$GEN/log/$a-body.log" 2>&1; echo "body exit $? $(grep -c '^left' "$GEN/log/$a-body.log") left; $(tail -2 "$GEN/log/$a-body.log" | head -1 | cut -c1-200)"
deep="$(python3 -c "import json,sys; d=json.load(open(sys.argv[1]))['asset']['dimensions']; print(round(max(0.015, 0.02*max(d['heightM'], d['widthM'], d['depthM']))*float(sys.argv[2]), 4))" "$src" "${DEEP:-0.5}")"
timeout 300 "$B" "$W/caps.ts" "$GEN/try/$a-raw-ops.json" "$W/body/$a.json" --deep "$deep" ${=CAPS:-$(cat "$W/flags/$a.caps" 2>/dev/null)} | tail -1 | sed "s|$W/||"
timeout 3000 "$B" "$U/🧰️matrix/MV-3/body.ts" "$src" --ops "$W/body/$a.json" --out "$GEN/try/$a.json" 2>&1 | tail -2 | cut -c1-200
timeout 1200 "$B" "$W/rest.ts" "$GEN/try/$a.json" > "$GEN/log/$a-try-rest.log" 2>&1; echo "rest exit $? $(tail -1 "$GEN/log/$a-try-rest.log")"
timeout 3000 "$B" "$M/gate.ts" "$GEN/try/$a.json" --soft > "$GEN/log/$a-try-gate.log" 2>&1; echo "gate exit $? $(tail -1 "$GEN/log/$a-try-gate.log")"
timeout 3000 "$B" "$U/🧰️matrix/MV-3/visible.ts" "$GEN/try/$a.json" --prefix mvb- > "$GEN/log/$a-try-visible.log" 2>&1; echo "visible exit $? $(tail -1 "$GEN/log/$a-try-visible.log" | cut -c1-260)"
