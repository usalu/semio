#!/bin/zsh
# 🛡️ The guarded dry run of one chain per asset in ONE grant of the memory queue (800 MB), nothing of the catalog is written: guard.sh <chain>:<id>…
#   fit     `settle.ts` (rebuild from `ops/<id>.json`; silhouette guard, validation, sweep as good as the backup) and `restbox.ts --drop` (no shared rest edge worse than the backup) in turn until neither removes an op; copy `$GEN/try/<id>.json`
#   nest    the same with `nsettle.ts` and `nest/<id>.json`; copy `$GEN/nest/<id>.json`
#   hybrid  `fit`, then the nest ops of the shapes the fit copy left flat on top of the fit copy (NEST_BASE), guarded the same way; copy `$GEN/nest/<id>.json`; the full nest proposal is kept in `nest/<id>.all.json`
# then the gate (poses, clip instants, transitions) and measure of the copy; one summary line per asset, logs `$GEN/guard/<id>.*`.
BATCH=MA source "${0:A:h}/../MR/env.sh"
export BATCH=MA
g="$GEN/guard"; mkdir -p "$g" "$GEN/nest" "$GEN/try" "$GEN/gate"
loop() {
  local kind="$1" a="$2" copy opsf tool
  if [[ "$kind" == fit ]]; then copy="$GEN/try/$a.json"; opsf="$W/ops/$a.json"; tool=settle.ts; else copy="$GEN/nest/$a.json"; opsf="$W/nest/$a.json"; tool=nsettle.ts; fi
  for i in 1 2 3 4 5 6 7 8; do
    (cd "$S/📦️packages/🟦️typescript" && timeout 4000 "$B" "$W/$tool" "$a" 2>&1 | cut -c1-400) >> "$g/$a.$kind.settle.log"
    (cd "$T" && timeout 600 "$B" "$W/restbox.ts" "$GEN/backup/$a.json" "$copy" --drop "$opsf") >> "$g/$a.$kind.rest.log" 2>&1 && break
  done
  echo "$kind: $(grep -c '"op"' "$opsf") ops, $i guard turn(s), rest: $(tail -1 "$g/$a.$kind.rest.log" | cut -c1-80)"
}
for pair in "$@"; do
  chain="${pair%%:*}"; a="${pair#*:}"; rm -f "$g/$a."*; unset NEST_BASE; line=""
  [[ -f "$W/nest/$a.all.json" ]] || { [[ -f "$W/nest/$a.json" ]] || "$W/nstage.sh" propose "$a" > "$g/$a.propose.out" 2>&1; cp "$W/nest/$a.json" "$W/nest/$a.all.json"; }
  if [[ "$chain" != nest ]]; then line+="$(loop fit "$a") | "; copy="$GEN/try/$a.json"; fi
  if [[ "$chain" == hybrid ]]; then
    python3 -c "
import json,sys
reps=json.load(open(sys.argv[1]))['asset']['representations']
flat=lambda o: any(s.get('id')==o['id'] and (o.get('bone') is None or s.get('bone')==o['bone']) and s.get('solid') is None for s in reps[o['scale']]['geometry'].get(o.get('view') or o.get('to') or '',[]))
ops=[o for o in json.load(open(sys.argv[2])) if o.get('op')!='nest' or flat(o)]
json.dump(ops,open(sys.argv[3],'w'),indent=1,ensure_ascii=False)" "$GEN/try/$a.json" "$W/nest/$a.all.json" "$W/nest/$a.json"
    export NEST_BASE="$GEN/try"
  fi
  if [[ "$chain" == nest ]]; then cp "$W/nest/$a.all.json" "$W/nest/$a.json"; fi
  if [[ "$chain" != fit ]]; then line+="$(loop nest "$a") | "; copy="$GEN/nest/$a.json"; fi
  (cd "$T" && timeout 1800 "$B" "$M/gate.ts" "$copy" --twins > "$g/$a.gate.log" 2>&1)
  (cd "$S/📦️packages/🟦️typescript" && timeout 900 "$B" "$M/measure.ts" "$GEN/backup/$a.json" "$copy" > "$g/$a.measure.log" 2>&1)
  echo "$chain" > "$g/$a.chain"
  echo "$a ($chain): $line$(tail -1 "$g/$a.gate.log") | $(grep -E 'silhouette:|validate:|sweep:' "$g/$a.measure.log" | tail -3 | tr '\n' ';')"
done
