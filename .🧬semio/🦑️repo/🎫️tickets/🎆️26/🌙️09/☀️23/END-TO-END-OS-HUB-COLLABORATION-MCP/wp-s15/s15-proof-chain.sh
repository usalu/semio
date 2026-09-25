#!/bin/zsh
# 🔍️ S15: live proof chain for catalog-generation resolution of hub documents on hub 8040 (catalog B), en + de:
# stale local staging (6541), a second session on the same device (store), a staged module identical to the bundle (front 6544, mirror),
# a plugin never staged (front 6542, unregister). Each run: create a hub document of one kind, open it, edit/undo/redo.
cd /Users/ueli/Documents/semio/.tmp-ticket/wp-s15 || exit 1
P="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s11-s15-profiles"
run() { local tag=$1 base=$2 index=$3 locale=$4 profile=$5 kind=$6 verb=$7
  echo "[chain] $tag start $(date '+%T')"
  S15_PROFILE_DIR="$P/$profile" S12_SPACE="S15 Diag 87969" S12_KIND_INDEX=$index S15_LOCALE=$locale bun s15-hub-journey.mjs "$base" "$tag" "$kind" "$verb" > "generated/s15-$tag-console.txt" 2>&1
  echo "[chain] $tag end $(date '+%T') rc=$?"; }
run draw-en-4 http://127.0.0.1:6541/ 1 en-US draw-en-4 draw none
run writer-en-3 http://127.0.0.1:6541/ 11 en-US writer-en-3 writer none
run note-de-1 http://127.0.0.1:6541/ 10 de-DE note-de-1 note addBlock
run draw-de-1 http://127.0.0.1:6541/ 1 de-DE draw-de-1 draw none
run writer-de-1 http://127.0.0.1:6541/ 11 de-DE writer-de-1 writer none
run note-en-store http://127.0.0.1:6541/ 10 en-US note-en-5 note addBlock
run note-en-mirror http://127.0.0.1:6544/ 10 en-US note-en-mirror note addBlock
run draw-en-unstaged http://127.0.0.1:6542/ 1 en-US draw-en-unstaged draw none
echo "[chain] done $(date '+%T')"
