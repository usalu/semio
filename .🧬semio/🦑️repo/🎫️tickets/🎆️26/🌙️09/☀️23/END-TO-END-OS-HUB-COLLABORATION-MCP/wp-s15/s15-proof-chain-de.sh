#!/bin/zsh
# 🔍️ S15: the de half of the hub-document resolution proof (note + draw on stale staging, draw never staged), hub 8040.
cd /Users/ueli/Documents/semio/.tmp-ticket/wp-s15 || exit 1
P="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s11-s15-profiles"
run() { local tag=$1 base=$2 index=$3 locale=$4 profile=$5 kind=$6 verb=$7
  echo "[chain] $tag start $(date '+%T')"
  S15_PROFILE_DIR="$P/$profile" S12_SPACE="S15 Diag 87969" S12_KIND_INDEX=$index S15_LOCALE=$locale bun s15-hub-journey.mjs "$base" "$tag" "$kind" "$verb" > "generated/s15-$tag-console.txt" 2>&1
  echo "[chain] $tag end $(date '+%T') rc=$?"; }
run note-de-2 http://127.0.0.1:6541/ 10 de-DE note-de-2 note addBlock
run draw-de-2 http://127.0.0.1:6541/ 1 de-DE draw-de-2 draw none
run draw-de-unstaged http://127.0.0.1:6542/ 1 de-DE draw-de-unstaged draw none
echo "[chain] done $(date '+%T')"
