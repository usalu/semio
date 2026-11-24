#!/bin/zsh
# 🔁️ Rebuilds the trial copy `$GEN/try/<id>.json` from the backup `$GEN/backup/<id>.json` (the asset before this batch) with `ops/<id>.json`: runs `fit.ts` until a run changes nothing, then asks `which.ts` for solids the evaluator cannot draw, marks their ops `"hull": true` and starts again (at most three rounds): refit.sh <id>
BATCH=MA source "${0:A:h}/../MR/env.sh"
a="$1"
for round in 1 2 3; do
  cp "$GEN/backup/$a.json" "$GEN/try/$a.json"
  for i in 1 2 3 4; do (cd "$T" && "$B" "$W/fit.ts" "$GEN/try/$a.json" --ops "$W/ops/$a.json" --turns "$GEN/turns-$a.json" --limit ${LIMIT:-0.75} --write > "$GEN/try/$a.fit$i.log" 2>&1); tail -1 "$GEN/try/$a.fit$i.log"; grep -q " 0 changed something" "$GEN/try/$a.fit$i.log" && break; done
  (cd "$T" && timeout 1500 "$B" "$W/which.ts" "$GEN/try/$a.json" > "$GEN/try/$a.which.jsonl" 2>&1)
  [[ -s "$GEN/try/$a.which.jsonl" ]] || break
  echo "evaluator cannot draw: $(cut -c1-150 "$GEN/try/$a.which.jsonl" | tr '\n' ' ')"
  [[ "$(python3 "$W/hull.py" "$W/ops/$a.json" "$GEN/try/$a.which.jsonl")" == 0 ]] && break
done
grep REFUSED "$GEN/try/$a.fit1.log" | cut -c1-240
