#!/bin/zsh
# 🔍️ S15: the canonical hub 7800 (catalog B) proof of hub-document resolution with stale local staging (serve 6543 → 7800), draw + writer,
# en + de. Waits (≤ 3 h) until 7800 serves the plugin module index with `extendsPluginId` (W2's os-hub rebuild), then runs.
cd /Users/ueli/Documents/semio/.tmp-ticket/wp-s15 || exit 1
for i in $(seq 1 360); do
  curl -s -m 5 http://127.0.0.1:7800/trusted-catalog/plugin-modules | /usr/bin/grep -q '"extendsPluginId"' && break
  sleep 30
done
curl -s -m 5 http://127.0.0.1:7800/trusted-catalog/plugin-modules | /usr/bin/grep -q '"extendsPluginId"' || { echo "[chain] 7800 never served the new index $(date '+%T')"; exit 1; }
echo "[chain] 7800 ready $(date '+%T')"
P="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s11-s15-profiles"
run() { local tag=$1 index=$2 locale=$3 kind=$4
  echo "[chain] $tag start $(date '+%T')"
  S15_PROFILE_DIR="$P/$tag" S12_SPACE="S15 Space 11208" S12_KIND_INDEX=$index S15_LOCALE=$locale bun s15-hub-journey.mjs http://127.0.0.1:6543/ "$tag" "$kind" none > "generated/s15-$tag-console.txt" 2>&1
  echo "[chain] $tag end $(date '+%T') rc=$?"; }
run draw-7800-en 1 en-US draw
run writer-7800-en 11 en-US writer
run draw-7800-de 1 de-DE draw
run writer-7800-de 11 de-DE writer
echo "[chain] done $(date '+%T')"
