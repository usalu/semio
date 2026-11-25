#!/bin/zsh
# 🧪️ One trial of an asset's ops on a copy: splits the lumped anchors of `anchors/<id>.json` if there is one (`split-anchors.py`), fits `ops/<id>.json` onto the backup (`fit2.ts`, run until nothing changes, at most four times) into $GEN/try/<id>.json, gates the copy on the subjects the baseline gate named (FULL=1: on every subject) and measures silhouette, validate and sweep of the backup and the copy: try.sh <id>   (GATE_ARGS and MEASURE_ARGS, e.g. `--scales 50`, narrow a trial)
export BATCH=MH-2; source "${0:A:h}/../MR/env.sh"
a="$1"; t="$GEN/try/$a.json"; mkdir -p "$GEN/try"
cp "$GEN/backup/$a.json" "$t"
[[ -f "$W/anchors/$a.json" ]] && python3 "$W/split-anchors.py" "$t" "$W/anchors/$a.json" --write
for i in 1 2 3 4; do
  ("$W/q.sh" 1000 600 "$T" "$B" "$W/fit2.ts" "$t" --ops "$W/ops/$a.json" --out "$t") > "$GEN/try/$a.fit$i.log" 2>&1
  grep -E "REFUSED" "$GEN/try/$a.fit$i.log" | cut -c1-300
  tail -1 "$GEN/try/$a.fit$i.log"
  grep -q " 0 changed something" "$GEN/try/$a.fit$i.log" && break
done
grep -oE "mismatch[^,]*[0-9.]+ mm.*" "$GEN/try/$a.fit1.log" | awk '{for(i=1;i<=NF;i++) if($i=="mm" && $(i-1)+0>25) {print "  large mismatch: " $0; break}}' | sort | uniq -c | head
only="$(python3 "$W/facts.py" "$GEN/gate0/$a.log" "$GEN/backup/$a.json" --subjects)"
if [[ -n "$FULL" ]]; then ("$W/q.sh" 2500 1800 "$T" "$B" "$W/gate2.ts" "$t" --twins --json "$GEN/try/$a.gate.json") > "$GEN/try/$a.gate.log" 2>&1
else ("$W/q.sh" 2500 1800 "$T" "$B" "$W/gate2.ts" "$t" --twins --only "$only" ${=GATE_ARGS} --json "$GEN/try/$a.gate.json") > "$GEN/try/$a.gate.log" 2>&1; fi
awk '/HARD/{p=1;print;next} /^[a-z]/{p=0} p' "$GEN/try/$a.gate.log" | cut -c1-240
tail -1 "$GEN/try/$a.gate.log"
if [[ -z "$NOMEASURE" ]]; then
  [[ -f "$GEN/backup/$a.measure" ]] || ("$W/q.sh" 2500 900 "$S/📦️packages/🟦️typescript" "$B" "$M/measure.ts" "$GEN/backup/$a.json") > "$GEN/backup/$a.measure" 2>&1
  ("$W/q.sh" 2500 900 "$S/📦️packages/🟦️typescript" "$B" "$M/measure.ts" "$t" ${=MEASURE_ARGS}) > "$GEN/try/$a.measure" 2>&1
  diff <(grep -vE "^   " "$GEN/backup/$a.measure") <(grep -vE "^   " "$GEN/try/$a.measure") && echo "measure: identical"
  grep -E "silhouette:|validate:|sweep:" "$GEN/try/$a.measure"
fi
