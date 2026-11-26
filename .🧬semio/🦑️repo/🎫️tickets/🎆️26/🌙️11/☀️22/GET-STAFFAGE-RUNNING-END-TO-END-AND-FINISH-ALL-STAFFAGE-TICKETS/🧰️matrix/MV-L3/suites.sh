#!/bin/zsh
# 🧪️ The six real test files with the name of one asset as filter, inside the grant of its caller (3000 MB: `🧊️three-views` of a tree peaked at 3026 MB in round MV-L2): catalog-silhouette, three-views, catalog-physics, breathing-catalog, asset-validation, asset-power; one line each into `$GEN/proof/<id>-suites.txt`. suites.sh <id>…
source "${0:A:h}/env.sh"
for a in "$@"; do
o="$GEN/proof/$a-suites.txt"; mkdir -p "$GEN/vt" "$GEN/proof"; echo "suites of $(date +%F\ %H:%M), file sha-256 $(shasum -a 256 < "$(ls "$C"/*/"$a"/🔣️.json)" | cut -c1-16)" > "$o"
vt() { local log="$GEN/vt/$a-$1.log" suite="$2"; shift 2; local pat="(${(j:|:)@})${SUFFIX-( |$)}"
  (cd "$S/📦️packages/🟦️typescript" && timeout 2400 "$B" x vitest run --config "../../🧪️tests/🎚️config/🟦️.ts" "../../🧪️tests/$suite/🟦️.ts" -t "$pat") > "$log" 2>&1
  echo "$suite exit $? $(grep -E '^ *(Test Files|Tests) ' "$log" | tr -s ' ' | tr '\n' ';')" >> "$o"; }
vt silhouette "🧊️catalog-silhouette" "$a"
vt three-views "🧊️three-views" "$a"
vt physics "🌡️catalog-physics" "$a"
SUFFIX=":" vt breathing "🫁️breathing-catalog" "$a"
SUFFIX="" vt validation "✅️asset-validation" "accepts the sample figure" "validates the manifest" "$a"
SUFFIX=":" vt power "⚡️asset-power" "$a"
echo "done $(date +%H:%M)" >> "$o"
done
