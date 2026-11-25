#!/bin/zsh
# 🩺️ The proof of one plant of batch MV-L1, one process after the other inside the reservation of its caller: it appends one line per check to `$GEN/proof/<id>.txt`. Checks: 1 gate over every subject (poses, clip instants, transitions), 2 official scan, 3 silhouette guard, `validateAsset` and three-views sweep outside the test runner (`../MR/measure.ts`: the backup first, then the catalog file), 4 check CLI with its suitability lines, 5 to 10 the real tests of the asset with a name filter (catalog-silhouette, three-views, catalog-physics, breathing-catalog, asset-validation, asset-power), 11 the matrix of the asset. FROM=<n> starts at check n and keeps the earlier lines; ONLY="1 2 3 4 11" runs those checks only (the suites 5 to 10 need 2500 MB, the others 1600 MB: the figures of MM in the takeover brief for a tree document of 1.5 to 6 MB, MATRIX_MB for another class). prove.sh <id>
source "${0:A:h}/env.sh"
a="$1"; d="$GEN/proof"; mkdir -p "$d" "$GEN/vt"; from="${FROM:-1}"; o="$d/$a.txt"; [[ "$from" -gt 1 || -n "$ONLY" ]] || : > "$o"
want() { [[ $1 -ge $from && ( -z "$ONLY" || " $ONLY " == *" $1 "* ) ]]; }
vt() { local log="$GEN/vt/$a-$1.log" suite="$2"; shift 2; local pat="(${(j:|:)@})${SUFFIX-( |$)}"
  "$W/q.sh" 2500 "$S/📦️packages/🟦️typescript" 1800 "$B" x vitest run --config "../../🧪️tests/🎚️config/🟦️.ts" "../../🧪️tests/$suite/🟦️.ts" -t "$pat" > "$log" 2>&1
  echo "$suite exit $? $(grep -E '^ *(Test Files|Tests) ' "$log" | tr -s ' ' | tr '\n' ';')" >> "$o"; }
want 1 && { "$W/q.sh" 800 "$T" 1800 "$B" "$M/gate.ts" "$a" --soft > "$d/$a-gate.log" 2>&1; echo "gate exit $? $(tail -1 "$d/$a-gate.log")" >> "$o"; }
want 2 && { "$W/q.sh" 800 "$T" 1800 "$B" 🔎️scan.ts --language en "$a" > "$d/$a-scan.log" 2>&1; echo "scan exit $? $(tail -1 "$d/$a-scan.log" | cut -c1-200)" >> "$o"; }
want 3 && { "$W/q.sh" 800 "$S/📦️packages/🟦️typescript" 2400 "$B" "$M/measure.ts" "$GEN/backup/$a.json" "$a" > "$d/$a-measure.log" 2>&1; grep -E "silhouette:|validate:|sweep:" "$d/$a-measure.log" >> "$o"; }
want 4 && { "$W/q.sh" 1000 "$S" 2400 "$B" "🔨️modules/🗂️catalog/🩺️check/🟦️.ts" --asset "$a" > "$d/$a-check.log" 2>&1; echo "check: exit $? $(grep -m1 -E '^(✓|✗)' "$d/$a-check.log" | cut -c1-160); $(grep -ciE 'prints|suitable' "$d/$a-check.log") suitability lines" >> "$o"; }
want 5 && vt silhouette "🧊️catalog-silhouette" "$a"
want 6 && vt three-views "🧊️three-views" "$a"
want 7 && vt physics "🌡️catalog-physics" "$a"
want 8 && SUFFIX=":" vt breathing "🫁️breathing-catalog" "$a"
want 9 && SUFFIX="" vt validation "✅️asset-validation" "accepts the sample figure" "validates the manifest" "$a"
want 10 && SUFFIX=":" vt power "⚡️asset-power" "$a"
if want 11; then
  rm -rf "$GEN/matrix-$a"; s=$(date +%s); "$W/q.sh" ${MATRIX_MB:-1600} "$T" ${MATRIX_TIMEOUT:-7200} "$B" 📊️matrix.ts --language en --out "$GEN/matrix-$a" --jobs 1 --fresh --assets "$a" > "$d/$a-matrix.log" 2>&1
  echo "matrix exit $? $(grep '\[matrix\].*cells:' "$d/$a-matrix.log" | tail -1) ($(( $(date +%s)-s )) s) $(python3 "$M/cells.py" "$GEN/matrix-$a" 2>&1 | tail -3 | tr '\n' ' ' | cut -c1-600)" >> "$o"
fi
echo "done $(date +%H:%M)" >> "$o"
