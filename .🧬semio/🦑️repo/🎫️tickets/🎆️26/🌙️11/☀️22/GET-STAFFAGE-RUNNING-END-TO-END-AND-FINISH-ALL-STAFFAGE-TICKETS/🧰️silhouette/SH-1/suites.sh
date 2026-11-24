#!/bin/zsh
# 🧪️ The other catalog suites with the name filter of the batch, one process after the other: suites.sh <tag> [slug…]  (logs 🗑️generated/silhouette/SH-1/vt/<tag>-<suite>.log). `asset-validation` and `asset-power` name no person of the batch in a test of its own: their catalog-wide tests are selected by name instead.
source "${0:A:h}/env.sh"
tag="$1"; shift
people=("$@"); [[ ${#people} -eq 0 ]] && people=($(cut -f2 "$U/🧰️silhouette/SH-1.tsv"))
echo "✅️asset-validation: $(SUFFIX="" "$W/vt.sh" "$tag-asset-validation" ✅️asset-validation "accepts the sample figure" "validates the manifest")"
echo "⚡️asset-power: $(SUFFIX="" "$W/vt.sh" "$tag-asset-power" ⚡️asset-power "is never negative" "keeps the manifest in step" "gives every rating a basis" "agrees with mathjs")"
echo "🫁️breathing-catalog: $(SUFFIX=":" "$W/vt.sh" "$tag-breathing-catalog" 🫁️breathing-catalog $people)"
[[ -n "$QUICK" ]] && exit 0
echo "🌡️catalog-physics: $(TMO=2400 "$W/vt.sh" "$tag-catalog-physics" 🌡️catalog-physics $people)"
echo "🧊️three-views: $(TMO=2400 "$W/vt.sh" "$tag-three-views" 🧊️three-views $people)"
