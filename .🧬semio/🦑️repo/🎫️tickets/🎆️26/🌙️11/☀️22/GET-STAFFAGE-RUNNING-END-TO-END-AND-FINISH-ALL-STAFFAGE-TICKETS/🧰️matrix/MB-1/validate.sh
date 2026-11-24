#!/bin/zsh
# 🩺️ `validateAsset` of the repaired asset against its original of 🗑️generated/matrix/MB-1/orig, one line per asset and the new issues: validate.sh <id>…
source "${0:A:h}/env.sh"
mkdir -p "$GEN/validate"; cd "$S/📦️packages/🟦️typescript"
for id in "$@"; do
  timeout 600 "$B" "$W/validate.ts" "$GEN/orig/$id.json" > "$GEN/validate/$id.before.log" 2>&1
  timeout 600 "$B" "$W/validate.ts" "$id" > "$GEN/validate/$id.after.log" 2>&1
  echo "$id validateAsset before: $(head -1 "$GEN/validate/$id.before.log" | cut -f2-) | after: $(head -1 "$GEN/validate/$id.after.log" | cut -f2-)"
  diff "$GEN/validate/$id.before.log" "$GEN/validate/$id.after.log" | grep '^>' | grep -v issues | cut -c1-${L:-260} | head -${N:-6}
done
