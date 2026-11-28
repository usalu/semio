#!/bin/zsh
# 🧪️ The real test files with the name of one asset as filter, inside the grant of its caller: catalog-silhouette, three-views, catalog-physics, the case of model-frame that pins the cow, asset-validation and, with the asset as the only selected one (`SEMIO_STAFFAGE_MODEL_ASSETS`, the command of the closing brief), model-catalog, whose findings `mine.py` sorts into the kinds of this lane and those of the lanes CX-GLB, CX-FBX and CX-ID; one line each into `$GEN/proof/<id>-suites.txt`. `ONLY=model-catalog` runs that one alone. suites.sh <id>…
source "${0:A:h}/env.sh"
for a in "$@"; do
o="$GEN/proof/$a-suites.txt"; mkdir -p "$GEN/vt" "$GEN/proof"; echo "suites of $(date +%F\ %H:%M), file sha-256 $(shasum -a 256 < "$(ls "$C"/*/"$a"/🔣️.json)" | cut -c1-16)" >> "$o"
vt() { local log="$GEN/vt/$a-$1.log" suite="$2"; shift 2; local pat="(${(j:|:)@})${SUFFIX-( |$)}"
  (cd "$S/📦️packages/🟦️typescript" && timeout 2400 "$B" x vitest run --config "../../🧪️tests/🎚️config/🟦️.ts" "../../🧪️tests/$suite/🟦️.ts" -t "$pat") > "$log" 2>&1
  echo "$suite exit $? $(grep -E '^ *(Test Files|Tests) ' "$log" | tr -s ' ' | tr '\n' ';')" >> "$o"; }
if [[ "${ONLY:-}" != model-catalog ]]; then
vt silhouette "🧊️catalog-silhouette" "$a"
vt three-views "🧊️three-views" "$a"
vt physics "🌡️catalog-physics" "$a"
[[ "$a" == dairy-cow ]] && SUFFIX="" vt frame "🧊️model-frame" "for dairy-cow"
SUFFIX="" vt validation "✅️asset-validation" "accepts the sample figure" "validates the manifest" "$a"
fi
(cd "$R" && SEMIO_STAFFAGE_MODEL_ASSETS="$a" timeout 3600 "$B" node_modules/vitest/vitest.mjs run --configLoader native --maxWorkers=1 --config "$S/🧪️tests/🎚️config/🟦️.ts" /🧊️model-catalog/ --testTimeout=3600000 --hookTimeout=3600000) > "$GEN/vt/$a-model-catalog.log" 2>&1
echo "🧊️model-catalog exit $? $(grep -E '^ *(Test Files|Tests) ' "$GEN/vt/$a-model-catalog.log" | tr -s ' ' | tr '\n' ';') $(python3 "$W/mine.py" "$GEN/vt/$a-model-catalog.log")" >> "$o"
echo "done $(date +%H:%M)" >> "$o"
done
