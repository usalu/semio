#!/bin/zsh
# 🌎️ W1: trusted catalog for every selectable `s` plugin.
#   phase warm  — per package, ONE wasm hold each: `nx run <p>-plugin:component-release` (the exact
#                 `cargo rustc --crate-type cdylib --profile wasm-release` unit the bootstrap links, via
#                 `pluginComponentRustcArgs`), so the final publish re-links nothing.
#   phase hub   — hub mutex: `cargo build --bin os-hub` in 🌎️hub/📦️packages/🦀️rust (the bootstrap's own build).
#   phase publish — ONE wasm hold: `trusted-catalog-bootstrap --packages all` into $DATA.
# usage: [W1_PACKAGES=a,b] [W1_DATA_NAME=dir] [W1_WARM_ONLY=a,b,…] zsh w1-catalog.sh <warm|hub|publish|all>
set -u
cd /Users/ueli/Documents/semio || exit 1
OUT=/Users/ueli/Documents/semio/.tmp-ticket/wp-w1/generated
MUTEX=(/Users/ueli/Documents/semio/.tmp-ticket/*fleet-mutex.sh)
HUB_RUST=(/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust)
SELECT=${W1_PACKAGES:-all}
DATA="/Users/ueli/Documents/semio/.🧬semio/🌐hub/${W1_DATA_NAME:-w1-catalog}"
unset CARGO_TARGET_DIR CARGO_BUILD_TARGET_DIR
export CARGO_INCREMENTAL=0
PACKAGES=(stdio gis writer draw puzzle animate architect block cad dag demonstrator energy fem flow forms imperative layout lowpoly mathematical norm note playbook procedural process raster reasoning remodel sequence shooting sourcing space trinity vcs wfc)
phase=${1:-all}
if [ "$phase" = warm ] || [ "$phase" = all ]; then
  WARM=("${PACKAGES[@]}"); [ -n "${W1_WARM_ONLY:-}" ] && WARM=(${(s:,:)W1_WARM_ONLY})
  for p in "${WARM[@]}"; do
    [ -f "$OUT/release-$p.ok" ] && { echo "[w1-catalog] SKIP release $p"; continue; }
    echo "[w1-catalog] START release $p $(date '+%F %T')"; s=$(date +%s)
    zsh "$MUTEX[1]" wasm w1 -- bun nx run "@semio-tech/$p-plugin:component-release" --outputStyle=stream > "$OUT/release-$p.txt" 2>&1
    rc=$?; echo "[w1-catalog] END release $p rc=$rc wall=$(( $(date +%s) - s ))s $(date '+%F %T')"
    [ $rc -eq 0 ] && touch "$OUT/release-$p.ok" || tail -12 "$OUT/release-$p.txt" | sed 's/^/    /'
  done
fi
if [ "$phase" = hub ] || [ "$phase" = all ]; then
  echo "[w1-catalog] START os-hub build $(date '+%F %T')"
  zsh "$MUTEX[1]" hub w1 -- zsh -c "cd '$HUB_RUST[1]' && cargo build --manifest-path Cargo.toml --bin os-hub" > "$OUT/hub-build.txt" 2>&1
  echo "[w1-catalog] END os-hub build rc=$? $(date '+%F %T')"
fi
if [ "$phase" = publish ] || [ "$phase" = all ]; then
  mkdir -p "$DATA"; chmod 700 "$DATA"
  echo "[w1-catalog] START publish $SELECT -> $DATA $(date '+%F %T')"; s=$(date +%s)
  zsh "$MUTEX[1]" wasm w1 -- env OS_HUB_DATA="$DATA" bun nx run os-hub:trusted-catalog-bootstrap --packages "$SELECT" --outputStyle=stream > "$OUT/publish-${W1_DATA_NAME:-w1-catalog}.txt" 2>&1
  prc=$?; echo "[w1-catalog] END publish rc=$prc wall=$(( $(date +%s) - s ))s $(date '+%F %T')"
fi
echo "[w1-catalog] DONE $phase $(date '+%F %T')"
exit ${prc:-0}
