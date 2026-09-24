#!/bin/zsh
# 🏁️ W2 final one-tree chain after the per-component stage chains (w2-stage-all.sh). Each wasm step is ONE fleet wasm-mutex hold:
#   1 generate (plugin-registry) → check      2 activate-s-react-dev (restage all s guests) → verify
#   3 component-release ×34 (one hold each)   4 os-hub build (hub mutex)   5 trusted-catalog-bootstrap --packages all → $DATA
# usage: zsh w2-final.sh [from-step]
set -u
cd /Users/ueli/Documents/semio || exit 1
W=/Users/ueli/Documents/semio/.tmp-ticket/wp-w2
OUT=$W/generated
MUTEX=(/Users/ueli/Documents/semio/.tmp-ticket/*fleet-mutex.sh)
HUB_RUST=(/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust)
DATA="/Users/ueli/Documents/semio/.🧬semio/🌐hub/w2-catalog-all"
unset CARGO_TARGET_DIR CARGO_BUILD_TARGET_DIR
export CARGO_INCREMENTAL=0 NX_DAEMON=false
from=${1:-1}
step() { echo "[w2-final] START $1 $(date '+%F %T')"; s=$(date +%s); }
done_() { echo "[w2-final] END $1 rc=$2 wall=$(( $(date +%s) - s ))s $(date '+%F %T')"; [ "$2" -eq 0 ] || { tail -30 "$3" | sed 's/^/    /'; exit "$2"; }; }
if [ $from -le 1 ]; then
  step generate; zsh "$MUTEX[1]" wasm w2 -- bun nx run @semio-tech/plugin-registry:generate --outputStyle=stream > "$OUT/final-1-generate.txt" 2>&1; done_ generate $? "$OUT/final-1-generate.txt"
  step check; bun nx run @semio-tech/plugin-registry:check --outputStyle=stream > "$OUT/final-1-check.txt" 2>&1; done_ check $? "$OUT/final-1-check.txt"
fi
if [ $from -le 2 ]; then
  step activate-s; zsh "$MUTEX[1]" wasm w2 -- bun nx run @semio-tech/framework-os-dev:activate-s-react-dev --outputStyle=stream > "$OUT/final-2-activate.txt" 2>&1; done_ activate-s $? "$OUT/final-2-activate.txt"
  step verify; bun "$W/w2-verify-staged.ts" > "$OUT/final-2-verify.txt" 2>&1; done_ verify $? "$OUT/final-2-verify.txt"
fi
if [ $from -le 3 ]; then
  for p in stdio gis animate architect block cad dag demonstrator draw energy fem flow forms imperative layout lowpoly mathematical norm note playbook procedural process puzzle raster reasoning remodel sequence shooting sourcing space trinity vcs wfc writer; do
    [ -f "$OUT/release-$p.ok" ] && continue
    step "release $p"; zsh "$MUTEX[1]" wasm w2 -- bun nx run "@semio-tech/$p-plugin:component-release" --outputStyle=stream > "$OUT/release-$p.txt" 2>&1
    rc=$?; echo "[w2-final] END release $p rc=$rc wall=$(( $(date +%s) - s ))s $(date '+%F %T')"; [ $rc -eq 0 ] && touch "$OUT/release-$p.ok" || tail -12 "$OUT/release-$p.txt" | sed 's/^/    /'
  done
fi
if [ $from -le 4 ]; then
  step hub-build; zsh "$MUTEX[1]" hub w2 -- zsh -c "cd '$HUB_RUST[1]' && cargo build --manifest-path Cargo.toml --bin os-hub" > "$OUT/final-4-hub-build.txt" 2>&1; done_ hub-build $? "$OUT/final-4-hub-build.txt"
fi
if [ $from -le 5 ]; then
  mkdir -p "$DATA"; chmod 700 "$DATA"
  step publish-all; zsh "$MUTEX[1]" wasm w2 -- env OS_HUB_DATA="$DATA" bun nx run os-hub:trusted-catalog-bootstrap --packages all --outputStyle=stream > "$OUT/final-5-publish.txt" 2>&1; done_ publish-all $? "$OUT/final-5-publish.txt"
fi
echo "[w2-final] DONE $(date '+%F %T')"
