#!/usr/bin/env bash
# 🩺️ Proves assets one after the other: the real `🧊️catalog-silhouette` test, `validateAsset`, the product check CLI and the other suites with the name filter; logs under 🗑️generated/matrix/MB-2/proof: prove.sh <tag> <id>…
source "$(dirname "$0")/env.sh"
tag="$1"; shift; mkdir -p "$G/proof"
for a in "$@"; do
  echo "$a | $("$M/vt.sh" "$tag-sil-$a" 🧊️catalog-silhouette "$a" | tr "\n" " ")"
  (cd "$S/📦️packages/🟦️typescript" && timeout 900 "$BUN" "$M/validate.ts" "$a") > "$G/proof/$tag-$a-validate.log" 2>&1
  (cd "$S" && timeout 1800 "$BUN" "🔨️modules/🗂️catalog/🩺️check/🟦️.ts" --asset "$a") > "$G/proof/$tag-$a-check.log" 2>&1
  echo "$a | validateAsset $(head -1 "$G/proof/$tag-$a-validate.log" | cut -f2,3) | check $(grep -m1 -E "^(✓|✗)" "$G/proof/$tag-$a-check.log" | cut -c1-160)"
  for suite in 🧊️three-views 🌡️catalog-physics 🫁️breathing-catalog; do echo "$a | $("$M/vt.sh" "$tag-$suite-$a" $suite "$a" | tr "\n" " ")"; done
  ids="(^|[ >])($a)([ :]|$)"
  echo "$a | $(PAT="$ids|manifest" "$M/vt.sh" "$tag-validation-$a" ✅️asset-validation | tr "\n" " ")"
  echo "$a | $(PAT="$ids|manifest|never negative|nameplate|every rating|engines|sorts the elements" "$M/vt.sh" "$tag-power-$a" ⚡️asset-power | tr "\n" " ")"
done
