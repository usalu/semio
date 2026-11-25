#!/bin/zsh
# 🧫️ The search run of a plant of batch MV-L1 on a copy, inside ONE reservation of the memory queue: `loft.ts` (or TOOL=body, the rigid carriers that were tried first) reads the catalog file, saves the chosen shapes to `ops/<id>.json` and writes the result to `$GEN/try/<id>.json`; the anchor search of batch MV-1 (`../MV-1/anchors.ts`) then puts the anchors the new drawing left behind back on the drawing of their bone (ops saved to `ops/<id>-anchors.json`); then the gate over every subject on that copy. Nothing in the catalog is written. trial.sh <megabytes> <id> [flags of body.ts…]   (ANCHORS="--steps 8" passes flags to the anchor search)
source "${0:A:h}/env.sh"
mb="$1"; a="$2"; shift 2; mkdir -p "$GEN/try" "$GEN/log" "$W/ops"; export MEMLOG="$GEN/log/mem-trial-$a.txt"
cd "$R" && INSIDE=1 repo capacity run --memory="$mb" -- zsh -c 'a="$1"; shift
"$W/q.sh" 0 "$T" 3000 "$B" "$W/${TOOL:-loft}.ts" "$a" --save "$W/ops/$a.json" --out "$GEN/try/$a.json" "$@" > "$GEN/log/$a-body.log" 2>&1; echo "body exit $? $(tail -3 "$GEN/log/$a-body.log" | cut -c1-200 | tr "\n" ";")"
"$W/q.sh" 0 "$T" 3000 "$B" "$W1/anchors.ts" "$GEN/try/$a.json" --save "$W/ops/$a-anchors.json" --out "$GEN/try/$a.json" ${=ANCHORS} > "$GEN/log/$a-anchors.log" 2>&1; echo "anchors exit $? $(tail -2 "$GEN/log/$a-anchors.log" | head -1 | cut -c1-200)"
"$W/q.sh" 0 "$T" 1800 "$B" "$M/gate.ts" "$GEN/try/$a.json" --soft > "$GEN/log/$a-gate.log" 2>&1; echo "gate exit $? $(tail -1 "$GEN/log/$a-gate.log")"' trial "$a" "$@"
