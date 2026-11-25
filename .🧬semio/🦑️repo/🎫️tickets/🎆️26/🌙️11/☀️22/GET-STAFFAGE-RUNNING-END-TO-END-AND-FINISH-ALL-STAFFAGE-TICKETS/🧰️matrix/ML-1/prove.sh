#!/bin/zsh
# 🩺️ The proof of assets after their repair under ONE grant of the memory queue (7000 MB, the largest step; the steps run one after the other in steps.sh): the authors' gate of the suite (`T/🔎️scan.ts`), `validateAsset`, whether every authored scale and range end prints, the product check CLI, the real tests with a name filter (catalog-silhouette, three-views, catalog-physics, breathing-catalog, asset-validation with the manifest cases, asset-power) and the coverage matrix per asset; the verdict lines go to 🗑️generated/matrix/ML-1/proof/summary-<tag>.txt and stdout: prove.sh <tag> <what,…> <id>…
source "${0:A:h}/env.sh"
tag="$1"; what="$2"; shift 2; mkdir -p "$GEN/proof"
qs ${MB:-7000} ${TO:-21600} "zsh '$W/steps.sh' '$what' $*" 2>&1 | grep -v '^\[capacity\]' | tee "$GEN/proof/summary-$tag.txt"
