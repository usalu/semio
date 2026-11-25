#!/bin/zsh
# 🩺️ The proof of plants of batch MV-1 after `make.sh`, one process after the other and every one through the machine-wide memory queue (`q.sh`: queue rule of 2026-11-25 09:20: 800 MB gate, scan and measure, 1000 MB check CLI, 2500 MB the vitest runs, 1200 MB matrix). It appends one line per check to `$GEN/proof/<id>.txt` and waits between two checks while the file `$GEN/pause` exists. Checks: 1 gate over every subject, 2 official scan, 3 silhouette, validate and sweep (measure.ts, the backup first, then the catalog file), 4 check CLI, 5 to 10 the real tests of the asset (catalog-silhouette, three-views, catalog-physics, breathing-catalog, asset-validation, asset-power), 11 with MATRIX=1 the matrix of the asset. FROM=<n> starts the first plant at check n and keeps its earlier lines (a chain that was stopped between two checks). prove.sh <id>…
export BATCH=MV-1; source "${0:A:h}/../MR/env.sh"
d="$GEN/proof"; mkdir -p "$d"; from="${FROM:-1}"
step() { while [[ -f "$GEN/pause" ]]; do touch "$GEN/paused"; sleep 3; done; rm -f "$GEN/paused"; [[ "$1" -ge "$from" ]]; }
for a in "$@"; do
  o="$d/$a.txt"; [[ "$from" -gt 1 ]] || : > "$o"
  step 1 && { "$W/q.sh" 800 "$T" 1800 "$B" "$M/gate.ts" "$a" --soft > "$d/$a-gate.log" 2>&1; echo "gate exit $? $(tail -1 "$d/$a-gate.log")" >> "$o"; }
  step 2 && { "$W/q.sh" 800 "$T" 1800 "$B" 🔎️scan.ts --language en "$a" > "$d/$a-scan.log" 2>&1; echo "scan exit $? $(tail -1 "$d/$a-scan.log" | cut -c1-200)" >> "$o"; }
  step 3 && { "$W/q.sh" 800 "$S/📦️packages/🟦️typescript" 2400 "$B" "$M/measure.ts" "$GEN/backup/$a.json" "$a" > "$d/$a-measure.log" 2>&1; grep -E "silhouette:|validate:|sweep:" "$d/$a-measure.log" >> "$o"; }
  step 4 && { "$W/q.sh" 1000 "$S" 2400 "$B" "🔨️modules/🗂️catalog/🩺️check/🟦️.ts" --asset "$a" > "$d/$a-check.log" 2>&1; echo "check: exit $? $(grep -m1 -E '^(✓|✗)' "$d/$a-check.log" | cut -c1-160); $(grep -ciE 'prints|suitable' "$d/$a-check.log") suitability lines" >> "$o"; }
  step 5 && "$W/vt.sh" "$a-silhouette" "🧊️catalog-silhouette" "$a" >> "$o"
  step 6 && "$W/vt.sh" "$a-three-views" "🧊️three-views" "$a" >> "$o"
  step 7 && "$W/vt.sh" "$a-physics" "🌡️catalog-physics" "$a" >> "$o"
  step 8 && SUFFIX=":" "$W/vt.sh" "$a-breathing" "🫁️breathing-catalog" "$a" >> "$o"
  step 9 && SUFFIX="" "$W/vt.sh" "$a-validation" "✅️asset-validation" "accepts the sample figure" "validates the manifest" "$a" >> "$o"
  step 10 && SUFFIX=":" "$W/vt.sh" "$a-power" "⚡️asset-power" "$a" >> "$o"
  if [[ -n "$MATRIX" ]] && step 11; then
    rm -rf "$GEN/matrix-$a"; s=$(date +%s); "$W/q.sh" 1200 "$T" ${MATRIX_TIMEOUT:-5400} "$B" 📊️matrix.ts --language en --out "$GEN/matrix-$a" --jobs 1 --fresh --assets "$a" > "$d/$a-matrix.log" 2>&1; echo "matrix exit $? $(grep '\[matrix\].*cells:' "$d/$a-matrix.log" | tail -1) ($(( $(date +%s)-s )) s with the wait in the queue) $(python3 "$M/cells.py" "$GEN/matrix-$a" 2>&1 | tail -3 | tr '\n' ' ')" >> "$o"
  fi
  echo "done $(date +%H:%M)" >> "$o"; from=1
done
