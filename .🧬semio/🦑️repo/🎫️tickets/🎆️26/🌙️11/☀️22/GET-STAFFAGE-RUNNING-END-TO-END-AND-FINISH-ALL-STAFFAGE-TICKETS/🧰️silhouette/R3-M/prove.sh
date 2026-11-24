#!/bin/zsh
# 🔬️ The proof of assets, one process after the other: the real `🧊️catalog-silhouette` test, `validateAsset`, the product check CLI and the asset's tests of the other suites by name filter; one line per step, logs in 🗑️generated/silhouette/R3-M/vt and …/proof: prove.sh <tag> <id>…   (SUITES="…" limits the further suites)
source "${0:A:h}/env.sh"
tag="$1"; shift; mkdir -p "$GEN/proof"
for a in "$@"; do
  echo "$a | silhouette | $(./vt.sh "$tag-sil-$a" 🧊️catalog-silhouette "$a")"
  echo "$a | validate | $(timeout 900 "$B" validate.ts "$a" 2>&1 | head -1 | cut -f2-)"
  (cd "$S" && timeout 900 "$B" "🔨️modules/🗂️catalog/🩺️check/🟦️.ts" --asset "$a") > "$GEN/proof/$tag-$a-check.log" 2>&1
  echo "$a | check | $(grep -m1 -E '^(✓|✗)' "$GEN/proof/$tag-$a-check.log" | cut -c1-160)"
  for suite in ${=SUITES-🧊️three-views 🌡️catalog-physics 🫁️breathing-catalog ✅️asset-validation ⚡️asset-power}; do
    echo "$a | $suite | $(SUFFIX= ./vt.sh "$tag-$suite-$a" "$suite" "$a")"
  done
done
