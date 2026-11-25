#!/bin/zsh
# 🧫️ Compares variants of a repair by the silhouette guard: var.sh <id> <scales> '<ops json>'… applies each ops list (JSON text) to the catalog file of the asset into $GEN/try/var/<id>.<n>.json and prints the measure lines that differ from the baseline; GATE="--scales 50" also gates each variant (hard findings only).
export BATCH=MH-6; source "${0:A:h}/../MR/env.sh"
a="$1"; scales="$2"; shift 2; mkdir -p "$GEN/try/var"; n=0
for ops in "$@"; do
  n=$((n+1)); f="$GEN/try/var/$a.$n"; print -r -- "$ops" > "$f.ops.json"
  echo "== variant $n: $(print -r -- "$ops" | cut -c1-150)"
  "$B" "$W/fit.ts" "${SRC:-$a}" --ops "$f.ops.json" --out "$f.json" | grep -E "REFUSED|ops," | cut -c1-200
  "$W/cap.sh" 800 1800 "$S/📦️packages/🟦️typescript" "$B" "$M/measure.ts" "$f.json" --scales "$scales" > "$f.measure.log" 2>&1; python3 "$W/cmp.py" "$GEN/measure0/$a.log" "$f.measure.log" | grep -E " 1:(${scales//,/|}) |validate|sweep"
  if [[ -n "$GATE" ]]; then "$W/cap.sh" 800 3600 "$T" "$B" "$M/gate.ts" "$f.json" --twins ${=GATE} > "$f.gate.log" 2>&1; python3 "$W/hard.py" "$f.gate.log" | cut -c1-170; fi
done
