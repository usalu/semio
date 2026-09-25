#!/bin/zsh
# 🛑️ S15: cancel of a hub program install from its band (fresh profile, writer), en + de. Waits for the eviction chain first.
cd /Users/ueli/Documents/semio/.tmp-ticket/wp-s15 || exit 1
while ! /usr/bin/grep -q "done" generated/s15-proof-chain-evict-1.txt 2>/dev/null; do sleep 5; done
P="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s11-s15-profiles"
for locale in en-US de-DE; do
  tag="writer-cancel-${locale%%-*}"
  echo "[chain] $tag start $(date '+%T')"
  S15_CANCEL_INSTALL=1 S15_PROFILE_DIR="$P/$tag" S12_SPACE="S15 Diag 87969" S12_KIND_INDEX=11 S15_LOCALE=$locale bun s15-hub-journey.mjs http://127.0.0.1:6541/ "$tag" writer none > "generated/s15-$tag-console.txt" 2>&1
  echo "[chain] $tag end $(date '+%T') rc=$?"
done
echo "[chain] done $(date '+%T')"
