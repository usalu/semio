#!/bin/zsh
# 🧪️ One trial of the repair of an asset on a copy: applies `ops/<id>.json` with `fit.ts` (the batch's own, which adds ops to MR's) to the catalog file into $GEN/try/<id>.json, runs it a second time on the copy (must change nothing), gates the copy (GATE_ARGS, e.g. "--scales 50 --poses") and measures silhouette, validate and sweep of the backup and the copy side by side: try.sh <id>   (NOGATE=1 skips the gate, NOMEASURE=1 the measure)
export BATCH=MH-6; source "${0:A:h}/../MR/env.sh"
a="$1"; mkdir -p "$GEN/try" "$GEN/trylog"
"$B" "$W/fit.ts" "$a" --ops "$W/ops/$a.json" --out "$GEN/try/$a.json" > "$GEN/trylog/$a.fit.log" 2>&1; echo "fit exit $? $(tail -1 "$GEN/trylog/$a.fit.log")"; grep -E "REFUSED|mismatch (side )?[3-9][0-9]\.|mismatch [0-9]{3}" "$GEN/trylog/$a.fit.log" | cut -c1-260
"$B" "$W/fit.ts" "$GEN/try/$a.json" --ops "$W/ops/$a.json" | tail -1
if [[ -z "$NOGATE" ]]; then (cd "$T" && timeout 1800 "$B" "$M/gate.ts" "$GEN/try/$a.json" --twins ${=GATE_ARGS}) > "$GEN/trylog/$a.gate.log" 2>&1; python3 "$W/hard.py" "$GEN/trylog/$a.gate.log"; fi
if [[ -z "$NOMEASURE" ]]; then (cd "$S/📦️packages/🟦️typescript" && timeout 900 "$B" "$M/measure.ts" "$GEN/try/$a.json" ${MEASURE_ARGS}) > "$GEN/trylog/$a.measure.log" 2>&1; python3 "$W/cmp.py" "$GEN/measure0/$a.log" "$GEN/trylog/$a.measure.log"; fi
