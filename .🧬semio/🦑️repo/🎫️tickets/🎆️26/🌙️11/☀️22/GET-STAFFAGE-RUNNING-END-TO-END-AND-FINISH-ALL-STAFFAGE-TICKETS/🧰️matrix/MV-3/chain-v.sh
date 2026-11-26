#!/bin/zsh
# 🧪️ The six real test files once each with the names of the given assets as filter, under one grant (`grant.sh 2900 <timeout> chain-v.sh <tag> <id>…`): catalog-silhouette, three-views, catalog-physics, breathing-catalog, asset-validation, asset-power. Lines go to `$GEN/proof/group-<tag>.txt`.
export BATCH=MV-3 MV3_GRANT=1; source "${0:A:h}/../MR/env.sh"
tag="$1"; shift; g="$GEN/proof/group-$tag.txt"; mkdir -p "$GEN/proof"; echo "tests of $(date +%F\ %H:%M) for $*" > "$g"
"$W/vt.sh" "$tag-silhouette" "🧊️catalog-silhouette" "$@" >> "$g"
"$W/vt.sh" "$tag-three-views" "🧊️three-views" "$@" >> "$g"
"$W/vt.sh" "$tag-physics" "🌡️catalog-physics" "$@" >> "$g"
SUFFIX=":" "$W/vt.sh" "$tag-breathing" "🫁️breathing-catalog" "$@" >> "$g"
SUFFIX="" "$W/vt.sh" "$tag-validation" "✅️asset-validation" "accepts the sample figure" "validates the manifest" "$@" >> "$g"
SUFFIX=":" "$W/vt.sh" "$tag-power" "⚡️asset-power" "$@" >> "$g"
echo "tests done $(date +%H:%M)" >> "$g"
