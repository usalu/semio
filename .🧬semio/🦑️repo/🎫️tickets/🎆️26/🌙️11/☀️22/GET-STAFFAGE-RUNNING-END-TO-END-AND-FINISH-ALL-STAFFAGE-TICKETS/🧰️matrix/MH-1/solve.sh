#!/bin/zsh
# 🧩️ One asset from its plan to a measured trial under ONE grant of the memory queue (2500 MB, the steps one after the other inside it): solve.ts settles plan/<id>.json against the silhouette guard into ops/<id>.json and $GEN/try/<id>.json (log $GEN/solve/<id>.log), then the full gate of the trial ($GEN/gate/try-<id>.log, hard findings printed) and the measures of the trial ($GEN/measure/try-<id>.log): solve.sh <id>…   (a file plan/<id>.probe with lines `<scale> <subject text, _ for a blank> <bone>…` adds the extents of single bones in one subject of the trial, `probe.ts`, into $GEN/solve/<id>.probe.log; SCAN="<id>…" first writes the full listing of the authors' scan of catalog assets into $GEN/now/scan-<id>.log; NOGATE=1 skips the gate; GATE_ARGS narrows it; inside the grant the script calls itself with INSIDE=1)
export BATCH=MH-1; source "${0:A:h}/../MR/env.sh"
mkdir -p "$GEN/solve" "$GEN/gate" "$GEN/measure" "$GEN/try" "$GEN/now"
if [[ -z "$INSIDE" ]]; then INSIDE=1 exec "$W/q.sh" ${SOLVE_MB:-800} ${SOLVE_TIMEOUT:-10800} "$S/📦️packages/🟦️typescript" "${0:A}" "$@"; fi
cd "$S/📦️packages/🟦️typescript"
for a in ${=SCAN}; do (cd "$T" && "$B" 🔎️scan.ts --language en "$a" > "$GEN/now/scan-$a.log" 2>&1); echo "scan $a: $(tail -1 "$GEN/now/scan-$a.log" | cut -c1-160)"; done
for a in "$@"; do
  echo "=== $a $(date +%H:%M)"
  /usr/bin/time -l -o "$GEN/solve/$a.time" "$B" "$W/solve.ts" "$GEN/backup/$a.json" --plan "$W/plan/$a${PLAN:+.$PLAN}.json" --ops "$W/ops/$a.json" --out "$GEN/try/$a.json" > "$GEN/solve/$a.log" 2>&1 || { echo "solve failed"; tail -5 "$GEN/solve/$a.log" | cut -c1-300; continue }
  grep -E "KEEP|NO candidate|refused|no measure|^1:|ops settled" "$GEN/solve/$a.log" | cut -c1-260
  (cd "$T" && "$B" "$M/../MH-4/scan-file.ts" "$GEN/try/$a.json" > "$GEN/gate/scan-$a.log" 2>&1); cut -c1-220 "$GEN/gate/scan-$a.log" | head -12
  if [[ -z "$NOGATE" ]]; then
    (cd "$T" && "$B" "$M/gate.ts" "$GEN/try/$a.json" --twins ${=GATE_ARGS} > "$GEN/gate/try-$a.log" 2>&1)
    awk '/ HARD /{p=1; print; next} /^    /{if(p) print; next} {p=0} /^hard findings/{print}' "$GEN/gate/try-$a.log" | cut -c1-240
  fi
  "$B" "$M/measure.ts" "$GEN/try/$a.json" > "$GEN/measure/try-$a.log" 2>&1; grep -E "silhouette:|validate:|sweep:" "$GEN/measure/try-$a.log"
  [[ -f "$W/plan/$a.probe" ]] && while read -r key text bones; do [[ -n "$key" ]] && (cd "$T" && "$B" "$W/probe.ts" "$GEN/try/$a.json" "$key" "${text//_/ }" ${=bones} 2>&1 | cut -c1-200); done < "$W/plan/$a.probe" > "$GEN/solve/$a.probe.log"
  echo "=== $a done $(date +%H:%M)"
done
