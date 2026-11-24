#!/bin/zsh
# 🔬️ The proof of assets of batch SF, one process at a time, each run to its end: the real `🧊️catalog-silhouette` test of the asset, `validateAsset`, the product check CLI, and the owned measure of every check; the lines also go to `🗑️generated/silhouette/SF/proof/<id>.txt`: prove.sh <id>…
source "${0:A:h}/env.sh"
mkdir -p "$GEN/proof"
for id in "$@"; do
  {
    echo "$id | catalog-silhouette: $(SUFFIX=' fills' TMO=${TMO:-1500} "$W/vt.sh" "sil-$id" "🧊️catalog-silhouette" "volume $id")"
    grep -E "AssertionError|Test timed out" "$GEN/vt/sil-$id.log" | head -3 | cut -c1-220
    echo "$id | validateAsset: $(cd "$W" && N=4 timeout 600 "$B" validate.ts "$id" 2>&1 | head -5 | cut -f2- | tr '\n' ' ' | cut -c1-400)"
    echo "$id | check CLI: $(cd "$S" && timeout 900 "$B" "🔨️modules/🗂️catalog/🩺️check/🟦️.ts" --asset "$id" 2>&1 | grep -m1 -E '^(✓|✗)' | cut -c1-160)"
    (cd "$W" && timeout 900 "$B" measure.ts "$id" 2>&1 | tail -1)
  } 2>&1 | tee "$GEN/proof/$id.txt"
done
