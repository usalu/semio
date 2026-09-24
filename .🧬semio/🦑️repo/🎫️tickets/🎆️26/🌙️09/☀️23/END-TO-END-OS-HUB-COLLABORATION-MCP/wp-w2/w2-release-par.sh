#!/bin/zsh
# 🧵️ W2 (coordinator 07:2x): component-release 3 at a time for distinct packages inside ONE wasm hold per batch, then catalog B
# (stdio,gis,note,animate,block,writer,draw,puzzle,wfc) on the rebuilt os-hub, then the rest, then the full catalog.
# Nx caches every package that succeeded, so a failed batch re-runs only the failed ones.
# usage: zsh w2-release-par.sh [from-phase: b-warm|b-publish|rest-warm|all-publish]
set -u
cd /Users/ueli/Documents/semio || exit 1
OUT="/Users/ueli/Documents/semio/.🧬semio/🌐hub/w2-logs"
MUTEX=(/Users/ueli/Documents/semio/.tmp-ticket/*fleet-mutex.sh)
HUB_RUST=(/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust)
unset CARGO_TARGET_DIR CARGO_BUILD_TARGET_DIR
export CARGO_INCREMENTAL=0 NX_DAEMON=false
PHASES=(b-warm b-publish rest-warm all-publish)
from=${1:-b-warm}; go=0
B_WARM=(block writer draw puzzle wfc)
REST=(dag raster architect cad demonstrator energy fem flow forms imperative layout lowpoly mathematical norm playbook procedural process reasoning remodel sequence shooting sourcing space trinity vcs)
warm() {
  local label=$1; shift
  local todo=(); for p in "$@"; do [ -f "$OUT/release-$p.ok" ] || todo+=("@semio-tech/$p-plugin"); done
  [ ${#todo[@]} -eq 0 ] && return 0
  for attempt in 1 2; do
    echo "[w2-par] START warm $label attempt=$attempt ${#todo[@]} packages $(date '+%F %T')"; local s=$(date +%s)
    zsh "$MUTEX[1]" wasm w2 -- bun nx run-many -t component-release --projects="${(j:,:)todo}" --parallel=3 --outputStyle=stream > "$OUT/release-par-$label-$attempt.txt" 2>&1
    local rc=$?; echo "[w2-par] END warm $label attempt=$attempt rc=$rc wall=$(( $(date +%s) - s ))s $(date '+%F %T')"
    if [ $rc -eq 0 ]; then for p in "$@"; do touch "$OUT/release-$p.ok"; done; return 0; fi
    grep -E 'error(\[E[0-9]+\])?:|Failed tasks|- @semio' "$OUT/release-par-$label-$attempt.txt" | head -12 | sed 's/^/    /'
    [ $attempt -eq 1 ] && sleep 300
  done
  return 1
}
publish() {
  local name=$1 packages=$2 DATA="/Users/ueli/Documents/semio/.🧬semio/🌐hub/$1"
  echo "[w2-par] START hub-build $(date '+%F %T')"
  zsh "$MUTEX[1]" hub w2 -- zsh -c "cd '$HUB_RUST[1]' && cargo build --manifest-path Cargo.toml --bin os-hub" > "$OUT/hub-build-$name.txt" 2>&1 || { echo "[w2-par] hub build failed"; return 1; }
  echo "[w2-par] END hub-build $(date '+%F %T')"
  mkdir -p "$DATA"; chmod 700 "$DATA"
  echo "[w2-par] START publish $name $(date '+%F %T')"; local s=$(date +%s)
  zsh "$MUTEX[1]" wasm w2 -- env OS_HUB_DATA="$DATA" bun nx run os-hub:trusted-catalog-bootstrap --packages "$packages" --outputStyle=stream > "$OUT/publish-$name.txt" 2>&1
  local rc=$?; echo "[w2-par] END publish $name rc=$rc wall=$(( $(date +%s) - s ))s $(date '+%F %T')"; return $rc
}
for phase in "${PHASES[@]}"; do
  [ "$phase" = "$from" ] && go=1; [ $go -eq 1 ] || continue
  case $phase in
    b-warm) warm b "${B_WARM[@]}" || exit 1 ;;
    b-publish) publish w2-catalog-b "stdio,gis,note,animate,block,writer,draw,puzzle,wfc" || exit 1 ;;
    rest-warm) warm rest "${REST[@]}" || exit 1 ;;
    all-publish) publish w2-catalog-all all || exit 1 ;;
  esac
done
echo "[w2-par] DONE $(date '+%F %T')"
