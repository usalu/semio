#!/bin/zsh
# 🧾️ The confirmation of assets after their repair, one process after the other: `validateAsset` (validate.ts), the product check CLI and the matrix of the asset alone (`T/📊️matrix.ts --fresh --assets <id>`, summed by the cells.py of MR); logs in $GEN/proof: confirm.sh <id>…   (MATRIX=0 leaves the matrix out)
source "${0:A:h}/env.sh"
d="$GEN/proof"; mkdir -p "$d"
for a in "$@"; do
  echo "$a validate: $(pb validate.ts "$a" 2>&1 | head -1 | cut -f2-)"
  (cd "$S" && timeout 900 "$B" "🔨️modules/🗂️catalog/🩺️check/🟦️.ts" --asset "$a") > "$d/$a-check.log" 2>&1; echo "$a check: $(grep -m1 -E '^(✓|✗)' "$d/$a-check.log" | cut -c1-120)"
  [[ "$MATRIX" == 0 ]] && continue
  rm -rf "$GEN/matrix-$a"; start=$SECONDS
  (cd "$T" && timeout ${MATRIX_TIMEOUT:-3000} "$B" 📊️matrix.ts --language en --out "$GEN/matrix-$a" --jobs 1 --fresh --assets "$a") > "$d/$a-matrix.log" 2>&1
  echo "$a matrix exit $? in $((SECONDS - start)) s: $(python3 "$M/cells.py" "$GEN/matrix-$a" 2>&1 | tr '\n' ' ' | cut -c1-400)"
done
