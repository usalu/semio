#!/bin/zsh
# 🧪️ The real test files with the name of one asset as filter, inside the grant of its caller: catalog-silhouette, three-views, catalog-physics, asset-validation and, with the asset as the only selected one (`SEMIO_STAFFAGE_MODEL_ASSETS`), model-catalog; one line each into `$GEN/proof/<id>-suites.txt`. suites.sh <id>…
source "${0:A:h}/env.sh"
for a in "$@"; do
o="$GEN/proof/$a-suites.txt"; mkdir -p "$GEN/vt" "$GEN/proof"; echo "suites of $(date +%F\ %H:%M), file sha-256 $(shasum -a 256 < "$(ls "$C"/*/"$a"/🔣️.json)" | cut -c1-16)" > "$o"
vt() { local log="$GEN/vt/$a-$1.log" suite="$2"; shift 2; local pat="(${(j:|:)@})${SUFFIX-( |$)}"
  (cd "$S/📦️packages/🟦️typescript" && timeout 2400 "$B" x vitest run --config "../../🧪️tests/🎚️config/🟦️.ts" "../../🧪️tests/$suite/🟦️.ts" -t "$pat") > "$log" 2>&1
  echo "$suite exit $? $(grep -E '^ *(Test Files|Tests) ' "$log" | tr -s ' ' | tr '\n' ';')" >> "$o"; }
vt silhouette "🧊️catalog-silhouette" "$a"
vt three-views "🧊️three-views" "$a"
vt physics "🌡️catalog-physics" "$a"
SUFFIX="" vt validation "✅️asset-validation" "accepts the sample figure" "validates the manifest" "$a"
SEMIO_STAFFAGE_MODEL_ASSETS="$a" vt model-catalog "🧊️model-catalog" "$a"
echo "done $(date +%H:%M)" >> "$o"
done
