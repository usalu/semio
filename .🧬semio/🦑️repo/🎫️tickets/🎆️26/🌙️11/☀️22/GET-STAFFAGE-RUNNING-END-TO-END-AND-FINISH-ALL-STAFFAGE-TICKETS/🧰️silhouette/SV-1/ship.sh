#!/bin/zsh
# 🚢️ Ships plants and proves them, one process after the other: the ops are replayed on the catalog file (apply.ts re-reads it right before it writes), then per plant the real `🧊️catalog-silhouette` test, `validateAsset` and the product check CLI; one line per proof, appended to 🗑️generated/silhouette/SV-1/proof.txt. ship.sh <id>…   (NOAPPLY=1 only proves)
source "${0:A:h}/env.sh"; cd "$W"
mkdir -p "$GEN/validate" "$GEN/check"
for id in "$@"; do
  {
    [[ -n "$NOAPPLY" || -f "$GEN/orig/$id.json" ]] || { mkdir -p "$GEN/orig"; cp "$CAT/$id/🔣️.json" "$GEN/orig/$id.json"; }
    [[ -n "$NOAPPLY" ]] || timeout 600 "$B" apply.ts "$id" | cut -c1-120
    ./vt.sh "ship-$id" 🧊️catalog-silhouette "$id"
    grep -E "^AssertionError" "$GEN/vt/ship-$id.log" | cut -c1-200 | head -3
    timeout 900 "$B" validate.ts "$id" "$GEN/validate/after-$id.txt" | head -4 | cut -c1-240
    (cd "$S" && timeout 900 "$B" "🔨️modules/🗂️catalog/🩺️check/🟦️.ts" --asset "$id" > "$GEN/check/$id.log" 2>&1; echo "check CLI exit $?: $(grep -m1 -E '^(✓|✗)' "$GEN/check/$id.log" | cut -c1-160)")
  } 2>&1 | tee -a "$GEN/proof.txt"
done
