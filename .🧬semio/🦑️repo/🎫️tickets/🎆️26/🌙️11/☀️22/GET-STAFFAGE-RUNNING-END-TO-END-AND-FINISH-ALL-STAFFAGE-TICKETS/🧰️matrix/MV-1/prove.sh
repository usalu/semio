#!/bin/zsh
# 🩺️ The proof of plants of batch MV-1 after `make.sh`, one process after the other (copy of ../MR/prove.sh that takes several ids, appends one line per check to `$GEN/proof/<id>.txt` and waits between two checks while the file `$GEN/pause` exists, so a short measurement can be run in between without two tool processes at once): gate over every subject, official scan, silhouette, validate and sweep (measure.ts against the backup), check CLI, the real tests of the asset (catalog-silhouette, three-views, catalog-physics, breathing-catalog, asset-validation, asset-power) and, with MATRIX=1, the matrix of the asset. prove.sh <id>…
export BATCH=MV-1; source "${0:A:h}/../MR/env.sh"
d="$GEN/proof"; mkdir -p "$d"
step() { while [[ -f "$GEN/pause" ]]; do touch "$GEN/paused"; sleep 3; done; rm -f "$GEN/paused"; }
for a in "$@"; do
  o="$d/$a.txt"; : > "$o"
  step; (cd "$T" && timeout 1800 "$B" "$M/gate.ts" "$a" --soft) > "$d/$a-gate.log" 2>&1; echo "gate exit $? $(tail -1 "$d/$a-gate.log")" >> "$o"
  step; (cd "$T" && timeout 1800 "$B" 🔎️scan.ts --language en "$a") > "$d/$a-scan.log" 2>&1; echo "scan exit $? $(tail -1 "$d/$a-scan.log" | cut -c1-200)" >> "$o"
  step; (cd "$S/📦️packages/🟦️typescript" && timeout 2400 "$B" "$M/measure.ts" "$GEN/backup/$a.json" "$a") > "$d/$a-measure.log" 2>&1; grep -E "silhouette:|validate:|sweep:" "$d/$a-measure.log" >> "$o"
  step; (cd "$S" && timeout 2400 "$B" "🔨️modules/🗂️catalog/🩺️check/🟦️.ts" --asset "$a") > "$d/$a-check.log" 2>&1; echo "check: exit $? $(grep -m1 -E '^(✓|✗)' "$d/$a-check.log" | cut -c1-160); $(grep -ciE 'prints|suitable' "$d/$a-check.log") suitability lines" >> "$o"
  step; "$M/vt.sh" "$a-silhouette" "🧊️catalog-silhouette" "$a" >> "$o"
  step; "$M/vt.sh" "$a-three-views" "🧊️three-views" "$a" >> "$o"
  step; "$M/vt.sh" "$a-physics" "🌡️catalog-physics" "$a" >> "$o"
  step; SUFFIX=":" "$M/vt.sh" "$a-breathing" "🫁️breathing-catalog" "$a" >> "$o"
  step; SUFFIX="" "$M/vt.sh" "$a-validation" "✅️asset-validation" "accepts the sample figure" "validates the manifest" "$a" >> "$o"
  step; SUFFIX=":" "$M/vt.sh" "$a-power" "⚡️asset-power" "$a" >> "$o"
  if [[ -n "$MATRIX" ]]; then
    step; rm -rf "$GEN/matrix-$a"; s=$(date +%s); (cd "$T" && timeout ${MATRIX_TIMEOUT:-5400} "$B" 📊️matrix.ts --language en --out "$GEN/matrix-$a" --jobs 1 --fresh --assets "$a") > "$d/$a-matrix.log" 2>&1; echo "matrix exit $? $(tail -1 "$d/$a-matrix.log") ($(( $(date +%s)-s )) s) $(python3 "$M/cells.py" "$GEN/matrix-$a" 2>&1 | tail -3 | tr '\n' ' ')" >> "$o"
  fi
  echo "done $(date +%H:%M)" >> "$o"
done
