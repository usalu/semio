#!/bin/zsh
# 🩺️ The proof of one asset after its repair (the steps of `../MR/prove.sh`), every run through the memory queue of the repo (q.sh: gate 2500 MB, scan 1000, measures 2500, check CLI 5000, each vitest run 2500, matrix 7000), one after the other, logs in $GEN/proof: prove.sh <id>   (MATRIX=1 adds `T/📊️matrix.ts --assets <id>`; SKIP="gate scan" leaves out steps already proven on the same file)
export BATCH=MH-1; source "${0:A:h}/../MR/env.sh"
a="$1"; d="$GEN/proof"; mkdir -p "$d"; want() { [[ " $SKIP " != *" $1 "* ]] }
want gate && { "$W/q.sh" 2500 7200 "$T" "$B" "$M/gate.ts" "$a" --twins > "$d/$a-gate.log" 2>&1; echo "gate exit $? $(tail -1 "$d/$a-gate.log")" }
want scan && { "$W/q.sh" 1000 3600 "$T" "$B" 🔎️scan.ts --language en "$a" > "$d/$a-scan.log" 2>&1; echo "scan exit $? $(tail -1 "$d/$a-scan.log" | cut -c1-200)" }
want measure && { "$W/q.sh" 2500 3600 "$S/📦️packages/🟦️typescript" "$B" "$M/measure.ts" "$a" > "$d/$a-measure.log" 2>&1; grep -E "silhouette:|validate:|sweep:" "$d/$a-measure.log" }
want check && { "$W/q.sh" 1000 5400 "$S" "$B" "🔨️modules/🗂️catalog/🩺️check/🟦️.ts" --asset "$a" > "$d/$a-check.log" 2>&1; echo "check: $(grep -m1 -E '^(✓|✗)' "$d/$a-check.log" | cut -c1-160)" }
want tests && {
  "$W/vt.sh" "$a-silhouette" "🧊️catalog-silhouette" "$a"
  "$W/vt.sh" "$a-three-views" "🧊️three-views" "$a"
  "$W/vt.sh" "$a-physics" "🌡️catalog-physics" "$a"
  SUFFIX=":" "$W/vt.sh" "$a-breathing" "🫁️breathing-catalog" "$a"
  SUFFIX="" "$W/vt.sh" "$a-validation" "✅️asset-validation" "accepts the sample figure" "validates the manifest" "$a"
  SUFFIX=":" "$W/vt.sh" "$a-power" "⚡️asset-power" "$a"
}
if [[ -n "$MATRIX" ]]; then
  rm -rf "$GEN/matrix-$a"; "$W/q.sh" 800 ${MATRIX_TIMEOUT:-10800} "$T" "$B" 📊️matrix.ts --language en --out "$GEN/matrix-$a" --jobs 1 --fresh --assets "$a" > "$d/$a-matrix.log" 2>&1; echo "matrix exit $? $(tail -1 "$d/$a-matrix.log")"
fi
