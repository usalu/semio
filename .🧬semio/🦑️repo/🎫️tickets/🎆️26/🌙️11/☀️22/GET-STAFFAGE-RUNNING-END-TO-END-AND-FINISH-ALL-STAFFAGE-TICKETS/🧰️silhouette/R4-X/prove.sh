#!/bin/zsh
# 🩺️ Applies the repair ops of objects to the catalog and proves each: the real `🧊️catalog-silhouette` test, `validateAsset`, the product check CLI and the other suites with the name filter; sheets of 1:50 and 1:500 after: prove.sh <id>…
source "${0:A:h}/env.sh"
mkdir -p "$GEN/proof"
for a in "$@"; do
  [[ -z "$NOAPPLY" && -f "$W/ops/$a.json" ]] && (cd "$S/📦️packages/🟦️typescript" && timeout 900 "$B" "$W/apply.ts" "$a")
  echo "$a | $("$W/vt.sh" "sil-$a" 🧊️catalog-silhouette "$a" | tr '\n' ' ')"
  (cd "$S/📦️packages/🟦️typescript" && N=12 timeout 900 "$B" "$W/validate.ts" "$a") > "$GEN/proof/$a-validate.log" 2>&1
  (cd "$S" && timeout 1800 "$B" "🔨️modules/🗂️catalog/🩺️check/🟦️.ts" --asset "$a") > "$GEN/proof/$a-check.log" 2>&1
  echo "$a | validateAsset $(head -1 "$GEN/proof/$a-validate.log" | cut -f2) | check $(grep -m1 -E '^(✓|✗)' "$GEN/proof/$a-check.log" | cut -c1-160)"
  "$W/others.sh" "after-$a" "$a" | sed "s/^/$a | /"
  for d in 50 500; do "$W/look.sh" "$(catfile $a)" "$a" $d after; done
done
