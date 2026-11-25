#!/bin/zsh
# 🩺️ The proof of one or more assets after their repair (the steps of `../MR/prove.sh`, each run through the memory queue, cap.sh): the gate over every pose, clip instant and transition, the official pose scan (T/🔎️scan.ts), silhouette, validate and three-views sweep (measure.ts), the product check CLI, the real tests of the asset in catalog-silhouette, three-views, catalog-physics, breathing-catalog, asset-validation and asset-power with the name filter, and with MATRIX=1 the matrix of the asset. ONLY="matrix" repeats the matrix alone. prove.sh <id>…   (every log in $GEN/proof, the lines in $GEN/proof/<id>.out)
source "${0:A:h}/env.sh"
source "$W/lock.sh"
d="$GEN/proof"; mkdir -p "$d" "$GEN/vt"
vt() {
  local log="$GEN/vt/$1.log" suite="$2"; shift 2
  local pat="(${(j:|:)@})${SUFFIX-( |$)}"
  "$W/cap.sh" 2500 1800 "$S/📦️packages/🟦️typescript" "$B" x vitest run --config "../../🧪️tests/🎚️config/🟦️.ts" "../../🧪️tests/$suite/🟦️.ts" -t "$pat" > "$log" 2>&1
  echo "$suite exit $? $(grep -E '^ *(Test Files|Tests) ' "$log" | tr -s ' ' | tr '\n' ';')"
}
for a in "$@"; do
  s0=$(date +%s)
  {
    if [[ "$ONLY" != matrix ]]; then
      "$W/cap.sh" 800 3600 "$T" "$B" "$M/gate.ts" "$a" --twins > "$d/$a-gate.log" 2>&1; echo "gate exit $? $(tail -1 "$d/$a-gate.log")"
      "$W/cap.sh" 800 1800 "$T" "$B" "$T/🔎️scan.ts" --language en "$a" > "$d/$a-scan.log" 2>&1; echo "scan exit $? $(tail -1 "$d/$a-scan.log" | cut -c1-200)"
      "$W/cap.sh" 1000 1800 "$S/📦️packages/🟦️typescript" "$B" "$M/measure.ts" "$a" > "$d/$a-measure.log" 2>&1; grep -E "silhouette:|validate:|sweep:" "$d/$a-measure.log"
      "$W/cap.sh" 1000 1800 "$S" "$B" "$S/🔨️modules/🗂️catalog/🩺️check/🟦️.ts" --asset "$a" > "$d/$a-check.log" 2>&1; echo "check: $(grep -m1 -E '^(✓|✗)' "$d/$a-check.log" | cut -c1-160)"
      "$W/cap.sh" 2500 10800 "$W" /bin/zsh "$W/vts.sh" "$a"
    fi
    if [[ -n "$MATRIX" || "$ONLY" == matrix ]]; then
      rm -rf "$GEN/matrix-$a"
      "$W/cap.sh" 800 ${MATRIX_TIMEOUT:-7200} "$T" "$B" "$T/📊️matrix.ts" --language en --out "$GEN/matrix-$a" --jobs 1 --fresh --assets "$a" > "$d/$a-matrix.log" 2>&1; echo "matrix exit $? $(tail -1 "$d/$a-matrix.log")"
      python3 "$M/cells.py" "$GEN/matrix-$a" 2>&1 | cut -c1-300
    fi
    echo "$a proof took $(( $(date +%s) - s0 )) s (through repo capacity run)"
  } 2>&1 | if [[ "$ONLY" == matrix ]]; then tee -a "$d/$a.out"; else tee "$d/$a.out"; fi
done
