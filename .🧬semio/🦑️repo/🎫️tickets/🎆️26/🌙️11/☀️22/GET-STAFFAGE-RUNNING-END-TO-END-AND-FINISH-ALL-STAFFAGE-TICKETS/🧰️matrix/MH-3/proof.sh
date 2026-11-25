#!/bin/zsh
# 🩺️ The proof of written assets, one process after the other, every log in $GEN/proof: that the catalog file is the dry run whose gate over every pose, clip instant and transition is logged in $GEN/gate/<id>.try.log (else the gate is run on the catalog file), the official pose scan, silhouette, validate and three-views sweep, the check CLI, and the real tests of the asset (catalog-silhouette, three-views, catalog-physics, breathing-catalog, asset-validation, asset-power): proof.sh <id>…   (MATRIX=1 adds `T/📊️matrix.ts --assets <id>` and its cell summary)
export BATCH=MH-3
here="${0:A:h}"
source "$here/../MR/env.sh"
B="$here/qbun"
d="$GEN/proof"; mkdir -p "$d"
for a in "$@"; do
  echo "== $a"
  if cmp -s "$S/🗂️catalog/👤️humans/$a/🔣️.json" "$GEN/try/$a.json" && grep -q "^hard findings: 0" "$GEN/gate/$a.try.log" && [[ $(grep -c " subjects " "$GEN/gate/$a.try.log") -ge 4 ]] && ! grep -q -- "--poses" "$GEN/gate/$a.flags" 2>/dev/null; then
    echo "gate: catalog file = dry run; $(tail -1 "$GEN/gate/$a.try.log") ($(grep " subjects " "$GEN/gate/$a.try.log" | sed -E 's/.* 1:([0-9]+) subjects ([0-9]+).*/1:\1 \2/' | tr '\n' ' ')subjects)"
  else
    (cd "$T" && QMEM=2500 QTIME=${GATE_TIMEOUT:-3000} "$B" "$M/gate.ts" "$a" --twins) > "$d/$a-gate.log" 2>&1; echo "gate exit $? $(tail -1 "$d/$a-gate.log")"
  fi
  (cd "$T" && QMEM=1000 QTIME=1800 "$B" 🔎️scan.ts --language en "$a") > "$d/$a-scan.log" 2>&1; echo "scan exit $? $(grep -m1 "^$a " "$d/$a-scan.log" | cut -c1-200)"
  (cd "$S/📦️packages/🟦️typescript" && QMEM=2500 QTIME=900 "$B" "$M/measure.ts" "$a") > "$d/$a-measure.log" 2>&1; grep -E "silhouette:|validate:|sweep:" "$d/$a-measure.log"
  (cd "$S/📦️packages/🟦️typescript" && QMEM=1000 QTIME=600 "$B" "$here/suit.ts" "$a") 2>&1 | grep -E "UNSUITABLE|^suitability:" | cut -c1-300
  (cd "$S" && QMEM=5000 QTIME=1800 "$B" "🔨️modules/🗂️catalog/🩺️check/🟦️.ts" --asset "$a") > "$d/$a-check.log" 2>&1; echo "check: $(grep -m1 -E '^(✓|✗)' "$d/$a-check.log" | cut -c1-160)"
  "$here/vt.sh" "$a-silhouette" "🧊️catalog-silhouette" "$a"
  "$here/vt.sh" "$a-three-views" "🧊️three-views" "$a"
  "$here/vt.sh" "$a-physics" "🌡️catalog-physics" "$a"
  SUFFIX=":" "$here/vt.sh" "$a-breathing" "🫁️breathing-catalog" "$a"
  SUFFIX="" "$here/vt.sh" "$a-validation" "✅️asset-validation" "accepts the sample figure" "validates the manifest" "$a"
  SUFFIX=":" "$here/vt.sh" "$a-power" "⚡️asset-power" "$a"
  if [[ -n "$MATRIX" ]]; then
    (cd "$T" && QMEM=7000 QTIME=${MATRIX_TIMEOUT:-7200} "$B" 📊️matrix.ts --language en --out "$GEN/matrix-$a" --jobs 1 --fresh --assets "$a") > "$d/$a-matrix.log" 2>&1; echo "matrix exit $? $(tail -1 "$d/$a-matrix.log" | cut -c1-200)"
    python3 "$M/cells.py" "$GEN/matrix-$a" 2>&1 | cut -c1-260
  fi
done
