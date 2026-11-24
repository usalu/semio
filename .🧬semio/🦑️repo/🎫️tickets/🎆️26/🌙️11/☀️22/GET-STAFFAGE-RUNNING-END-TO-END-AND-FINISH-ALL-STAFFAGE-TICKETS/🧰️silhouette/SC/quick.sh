#!/bin/zsh
# ⚡️ The quick half of the proof of components: `validateAsset` and the product check CLI (compared with the suitability lines of the original file kept in 🗑️generated/silhouette/SC/orig): quick.sh <id>…
source "${0:A:h}/env.sh"
mkdir -p "$GEN/quick"
cd "$S/📦️packages/🟦️typescript"
for id in "$@"; do
  echo "$id validateAsset: $(L=300 timeout 300 "$B" "$W/validate.ts" "$id" 2>&1 | head -4 | cut -c1-300)"
  timeout 900 "$B" "$S/🔨️modules/🗂️catalog/🩺️check/🟦️.ts" --asset "$id" > "$GEN/quick/$id.after.log" 2>&1; echo "$id check exit $?"
  [[ -f "$GEN/quick/$id.before.log" ]] || timeout 900 "$B" "$S/🔨️modules/🗂️catalog/🩺️check/🟦️.ts" --file "$GEN/orig/$id.json" > "$GEN/quick/$id.before.log" 2>&1
  diff <(sed 's/^✓ .*/✓/' "$GEN/quick/$id.before.log") <(sed 's/^✓ .*/✓/' "$GEN/quick/$id.after.log") | cut -c1-260 | head -12
done
