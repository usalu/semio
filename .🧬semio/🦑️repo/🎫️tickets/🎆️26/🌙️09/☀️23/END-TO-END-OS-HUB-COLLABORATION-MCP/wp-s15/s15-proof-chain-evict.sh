#!/bin/zsh
# 🧹️ S15: store eviction proof — one stored file of note's bundle deleted before a hub note opens: localized reinstall notice,
# only the evicted file downloaded again, en (profile note-en-5) + de (profile note-de-2). Waits for the de chain first.
cd /Users/ueli/Documents/semio/.tmp-ticket/wp-s15 || exit 1
while ! /usr/bin/grep -q "done" generated/s15-proof-chain-de-1.txt 2>/dev/null; do sleep 5; done
P="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s11-s15-profiles"
run() { local tag=$1 locale=$2 profile=$3
  echo "[chain] $tag start $(date '+%T')"
  S15_EVICT=note S15_PROFILE_DIR="$P/$profile" S12_SPACE="S15 Diag 87969" S12_KIND_INDEX=10 S15_LOCALE=$locale bun s15-hub-journey.mjs http://127.0.0.1:6541/ "$tag" note addBlock > "generated/s15-$tag-console.txt" 2>&1
  echo "[chain] $tag end $(date '+%T') rc=$?"; }
run note-en-evict en-US note-en-5
run note-de-evict de-DE note-de-2
echo "[chain] done $(date '+%T')"
