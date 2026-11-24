#!/bin/zsh
# 🩺️ The proof of one asset after its repair, one process after the other, every log in $GEN/proof: the gate over every pose, clip instant and transition (gate.ts), the official pose scan (T/🔎️scan.ts), silhouette, validate and three-views sweep (measure.ts), the product check CLI, and the real tests of the asset in catalog-silhouette, three-views, catalog-physics, breathing-catalog, asset-validation (the sample figure, the manifest cases and any case naming the asset) and asset-power (tests named `<id>: …`; an asset without a case there shows as skipped). prove.sh <id>   (MATRIX=1 adds `T/📊️matrix.ts --assets <id>`, slow)
source "${0:A:h}/env.sh"
a="$1"; d="$GEN/proof"; mkdir -p "$d"
(cd "$T" && timeout 1800 "$B" "$M/gate.ts" "$a" --twins) > "$d/$a-gate.log" 2>&1; echo "gate exit $? $(tail -1 "$d/$a-gate.log")"
(cd "$T" && timeout 1800 "$B" 🔎️scan.ts --language en "$a") > "$d/$a-scan.log" 2>&1; echo "scan exit $? $(tail -1 "$d/$a-scan.log" | cut -c1-200)"
(cd "$S/📦️packages/🟦️typescript" && timeout 900 "$B" "$M/measure.ts" "$a") > "$d/$a-measure.log" 2>&1; grep -E "silhouette:|validate:|sweep:" "$d/$a-measure.log"
(cd "$S" && timeout 900 "$B" "🔨️modules/🗂️catalog/🩺️check/🟦️.ts" --asset "$a") > "$d/$a-check.log" 2>&1; echo "check: $(grep -m1 -E '^(✓|✗)' "$d/$a-check.log" | cut -c1-160)"
"$M/vt.sh" "$a-silhouette" "🧊️catalog-silhouette" "$a"
"$M/vt.sh" "$a-three-views" "🧊️three-views" "$a"
"$M/vt.sh" "$a-physics" "🌡️catalog-physics" "$a"
SUFFIX=":" "$M/vt.sh" "$a-breathing" "🫁️breathing-catalog" "$a"
SUFFIX="" "$M/vt.sh" "$a-validation" "✅️asset-validation" "accepts the sample figure" "validates the manifest" "$a"
SUFFIX=":" "$M/vt.sh" "$a-power" "⚡️asset-power" "$a"
if [[ -n "$MATRIX" ]]; then
  rm -rf "$GEN/matrix-$a"; (cd "$T" && timeout ${MATRIX_TIMEOUT:-5400} "$B" 📊️matrix.ts --language en --out "$GEN/matrix-$a" --jobs 1 --fresh --assets "$a") > "$d/$a-matrix.log" 2>&1; echo "matrix exit $? $(tail -1 "$d/$a-matrix.log")"
fi
