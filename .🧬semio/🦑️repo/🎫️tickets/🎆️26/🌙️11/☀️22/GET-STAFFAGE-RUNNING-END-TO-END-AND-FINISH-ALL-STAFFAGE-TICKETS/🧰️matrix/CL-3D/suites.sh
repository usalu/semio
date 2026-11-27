#!/bin/zsh
# 🧪️ Real test files with the name of one asset as filter, inside the grant of its caller, one line each into `$GEN/proof/<id>-suites.txt`: by default model-catalog with the asset as the only selected one (`SEMIO_STAFFAGE_MODEL_ASSETS`); with `ALL=1` before it catalog-silhouette, three-views, catalog-physics and asset-validation. suites.sh <id>…
source "${0:A:h}/env.sh"
for a in "$@"; do
o="$GEN/proof/$a-suites.txt"; mkdir -p "$GEN/vt" "$GEN/proof"; echo "suites of $(date +%F\ %H:%M), file sha-256 $(shasum -a 256 < "$(ls "$C"/*/"$a"/🔣️.json)" | cut -c1-16)" > "$o"
vt() { local log="$GEN/vt/$a-$1.log" suite="$2"; shift 2; local pat="(${(j:|:)@})${SUFFIX-( |$)}"
  (cd "$S/📦️packages/🟦️typescript" && timeout 3000 "$B" x vitest run --config "../../🧪️tests/🎚️config/🟦️.ts" "../../🧪️tests/$suite/🟦️.ts" -t "$pat" --testTimeout=2400000) > "$log" 2>&1
  echo "$suite exit $? $(grep -E '^ *(Test Files|Tests) ' "$log" | tr -s ' ' | tr '\n' ';') $(grep -aE '^\+ +"(budget|bounds)' "$log" | tr -s ' ' | cut -c1-110 | tr '\n' ';' | cut -c1-500)" >> "$o"; }
if [[ -n "$ALL" ]]; then
  vt silhouette "🧊️catalog-silhouette" "$a"
  vt three-views "🧊️three-views" "$a"
  vt physics "🌡️catalog-physics" "$a"
  SUFFIX="" vt validation "✅️asset-validation" "accepts the sample figure" "validates the manifest" "$a"
fi
SEMIO_STAFFAGE_MODEL_ASSETS="$a" vt model-catalog "🧊️model-catalog" "$a"
echo "done $(date +%H:%M)" >> "$o"
done
