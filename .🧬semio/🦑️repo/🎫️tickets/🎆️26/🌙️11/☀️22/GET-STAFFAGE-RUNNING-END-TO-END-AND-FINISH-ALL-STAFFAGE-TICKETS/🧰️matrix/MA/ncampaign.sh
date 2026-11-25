#!/bin/zsh
# 🏕️ The whole dry run of the nested-prism repair over several assets as ONE process tree (start it through the memory queue: `repo capacity run --memory=2500 -- timeout <s> ncampaign.sh <id>…`), nothing of the catalog is written: `nstage.sh propose` where `nest/<id>.json` does not exist yet, the settle loop `nsettle.ts` (silhouette guard, validation, three-views sweep and evaluation as good as the backup), then gate and measure of the settled trial copy; one summary line per asset, logs in `$GEN/nest`.
BATCH=MA source "${0:A:h}/../MR/env.sh"
export BATCH=MA
mkdir -p "$GEN/nest"
for a in "$@"; do
  [[ -f "$W/nest/$a.json" ]] || "$W/nstage.sh" propose "$a" > "$GEN/nest/$a.propose.out" 2>&1
  (cd "$S/📦️packages/🟦️typescript" && timeout ${SETTLE_TIMEOUT:-4000} "$B" "$W/nsettle.ts" "$a" > "$GEN/nest/$a.settle.log" 2>&1)
  "$W/nstage.sh" gate "$a" > "$GEN/nest/$a.gate.out" 2>&1
  "$W/nstage.sh" measure "$a" > "$GEN/nest/$a.measure.out" 2>&1
  echo "$a: $(grep -c '"op"' "$W/nest/$a.json") ops | settle: $(grep -c '^\[round' "$GEN/nest/$a.settle.log") rounds, last: $(grep '^\[round' "$GEN/nest/$a.settle.log" | tail -1 | sed 's/.*| failing: //' | cut -c1-200) | $(tail -1 "$GEN/nest/$a.gate.log") | $(grep -E 'silhouette:|validate:|sweep:' "$GEN/nest/$a.measure.log" | tail -3 | tr '\n' ';')"
done
