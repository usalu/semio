#!/bin/zsh
# 🧫️ The six test suites of one asset with the name filter, one after the other under ONE 2500 MB grant (prove.sh starts it through cap.sh; the suites then run directly): vts.sh <id>   prints one line per suite, the logs go to $GEN/vt.
source "${0:A:h}/env.sh"
export MH4_GRANT=2500
a="$1"; mkdir -p "$GEN/vt"
vt() {
  local log="$GEN/vt/$1.log" suite="$2"; shift 2
  local pat="(${(j:|:)@})${SUFFIX-( |$)}"
  "$W/cap.sh" 2500 1800 "$S/📦️packages/🟦️typescript" "$B" x vitest run --config "../../🧪️tests/🎚️config/🟦️.ts" "../../🧪️tests/$suite/🟦️.ts" -t "$pat" > "$log" 2>&1
  echo "$suite exit $? $(grep -E '^ *(Test Files|Tests) ' "$log" | tr -s ' ' | tr '\n' ';')"
}
vt "$a-silhouette" "🧊️catalog-silhouette" "$a"
vt "$a-three-views" "🧊️three-views" "$a"
vt "$a-physics" "🌡️catalog-physics" "$a"
SUFFIX=":" vt "$a-breathing" "🫁️breathing-catalog" "$a"
SUFFIX="" vt "$a-validation" "✅️asset-validation" "accepts the sample figure" "validates the manifest" "$a"
SUFFIX=":" vt "$a-power" "⚡️asset-power" "$a"
