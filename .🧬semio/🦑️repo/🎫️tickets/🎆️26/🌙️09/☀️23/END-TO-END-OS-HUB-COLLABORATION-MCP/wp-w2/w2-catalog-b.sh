#!/bin/zsh
# 🅱️ W2 interim catalog B from the same tree as the full catalog: warm the collaboration/inference packages first (one wasm hold each,
# shared .ok markers with w2-final.sh), build os-hub (hub mutex), publish stdio,gis,note,draw,writer,puzzle,wfc,block → w2-catalog-b.
set -u
cd /Users/ueli/Documents/semio || exit 1
OUT=/Users/ueli/Documents/semio/.tmp-ticket/wp-w2/generated
MUTEX=(/Users/ueli/Documents/semio/.tmp-ticket/*fleet-mutex.sh)
HUB_RUST=(/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust)
DATA="/Users/ueli/Documents/semio/.🧬semio/🌐hub/w2-catalog-b"
unset CARGO_TARGET_DIR CARGO_BUILD_TARGET_DIR
export CARGO_INCREMENTAL=0 NX_DAEMON=false
for p in note draw writer puzzle wfc block; do
  [ -f "$OUT/release-$p.ok" ] && continue
  echo "[w2-b] START release $p $(date '+%F %T')"; s=$(date +%s)
  zsh "$MUTEX[1]" wasm w2 -- bun nx run "@semio-tech/$p-plugin:component-release" --outputStyle=stream > "$OUT/release-$p-b.txt" 2>&1
  rc=$?; echo "[w2-b] END release $p rc=$rc wall=$(( $(date +%s) - s ))s $(date '+%F %T')"; [ $rc -eq 0 ] && touch "$OUT/release-$p.ok" || { tail -12 "$OUT/release-$p-b.txt"; exit 1; }
done
until [ -f "$OUT/release-stdio.ok" ] && [ -f "$OUT/release-gis.ok" ]; do sleep 30; done
echo "[w2-b] START hub-build $(date '+%F %T')"
zsh "$MUTEX[1]" hub w2 -- zsh -c "cd '$HUB_RUST[1]' && cargo build --manifest-path Cargo.toml --bin os-hub" > "$OUT/hub-build-b.txt" 2>&1 || { echo "[w2-b] hub build failed"; exit 1; }
echo "[w2-b] END hub-build $(date '+%F %T')"
mkdir -p "$DATA"; chmod 700 "$DATA"
echo "[w2-b] START publish $(date '+%F %T')"; s=$(date +%s)
zsh "$MUTEX[1]" wasm w2 -- env OS_HUB_DATA="$DATA" bun nx run os-hub:trusted-catalog-bootstrap --packages "stdio,gis,note,draw,writer,puzzle,wfc,block" --outputStyle=stream > "$OUT/publish-w2-catalog-b.txt" 2>&1
rc=$?; echo "[w2-b] END publish rc=$rc wall=$(( $(date +%s) - s ))s $(date '+%F %T')"
exit $rc
