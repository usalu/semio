#!/bin/zsh
# 🛣️ W2 (session 12): component-release in N real lanes inside ONE wasm hold, then the catalog publishes.
# `component-release` is inferred with `parallelism: false`, so `nx run-many --parallel=N` runs one package at a time;
# real parallelism needs N concurrent Nx processes (each lane builds its packages in order). A package is ok when its
# `dist/component-release/semio_s_plugin_<p>.wasm` was rewritten after the lane hold started (measured, not assumed).
# Lock-cycle risk (concurrent cargos on the shared build-dir): W2 watches for lane cargos at 0 % CPU with no rustc child.
# usage: zsh w2-release-lanes.sh <from-phase: b-lanes|b-publish|rest-lanes|all-publish> [lanes=3]
set -u
cd /Users/ueli/Documents/semio || exit 1
OUT="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s12-w2-logs"
MUTEX=(/Users/ueli/Documents/semio/.tmp-ticket/*fleet-mutex.sh)
HUB_RUST=(/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust)
unset CARGO_TARGET_DIR CARGO_BUILD_TARGET_DIR
export CARGO_INCREMENTAL=0 NX_DAEMON=false
PHASES=(b-lanes b-publish rest-lanes all-publish)
from=${1:-b-lanes}; N=${2:-3}; go=0
B=(gis writer puzzle note draw block animate wfc stdio)
REST=(demonstrator procedural cad sourcing raster remodel lowpoly energy architect space trinity flow layout process fem norm forms shooting dag imperative mathematical playbook reasoning sequence vcs)
release_path() { local f=(✏️s/🔌️plugins/*/📦️packages/🦀️rust/dist/component-release/semio_s_plugin_$1.wasm(N)); print -r -- "${f[1]:-}"; }
lanes() {
  local label=$1; shift
  local todo=(); for p in "$@"; do [ -f "$OUT/release-$p.ok" ] || todo+=("$p"); done
  [ ${#todo[@]} -eq 0 ] && return 0
  local stamp="$OUT/lanes-$label.stamp"; : > "$stamp"
  local cmd="" i=0
  for i in $(seq 1 $N); do
    local lane=(); local j=$i; while [ $j -le ${#todo[@]} ]; do lane+=("$todo[$j]"); j=$(( j + N )); done
    [ ${#lane[@]} -eq 0 ] && continue
    local pr=(); for p in "${lane[@]}"; do pr+=("@semio-tech/$p-plugin"); done
    cmd+="bun nx run-many -t component-release --projects=${(j:,:)pr} --parallel=1 --outputStyle=stream > '$OUT/release-lanes-$label-$i.txt' 2>&1 & "
  done
  echo "[w2-lanes] START $label lanes=$N ${#todo[@]} packages ${todo[*]} $(date '+%F %T')"; local s=$(date +%s)
  zsh "$MUTEX[1]" wasm w2 -- zsh -c "setopt no_bg_nice; ${cmd} wait"
  echo "[w2-lanes] END $label wall=$(( $(date +%s) - s ))s $(date '+%F %T')"
  local failed=()
  for p in "${todo[@]}"; do
    local r=$(release_path $p)
    if [ -n "$r" ] && [ "$r" -nt "$stamp" ]; then touch "$OUT/release-$p.ok"; echo "    ok $p $(stat -f '%Sm' -t '%T' "$r") $(shasum -a 256 "$r" | cut -c1-16)"; else failed+=("$p"); fi
  done
  [ ${#failed[@]} -eq 0 ] && return 0
  echo "[w2-lanes] FAILED $label: ${failed[*]}"; /usr/bin/grep -hE 'error(\[E[0-9]+\])?:|Failed tasks|- @semio' "$OUT"/release-lanes-$label-*.txt | head -20 | sed 's/^/    /'
  return 1
}
publish() {
  local name=$1 packages=$2 DATA="/Users/ueli/Documents/semio/.🧬semio/🌐hub/$1"
  echo "[w2-lanes] START hub-build $(date '+%F %T')"
  zsh "$MUTEX[1]" hub w2 -- zsh -c "cd '$HUB_RUST[1]' && cargo build --manifest-path Cargo.toml --bin os-hub" > "$OUT/hub-build-$name.txt" 2>&1 || { echo "[w2-lanes] hub build failed"; return 1; }
  echo "[w2-lanes] END hub-build $(date '+%F %T')"
  mkdir -p "$DATA"; chmod 700 "$DATA"
  echo "[w2-lanes] START publish $name $(date '+%F %T')"; local s=$(date +%s)
  zsh "$MUTEX[1]" wasm w2 -- env OS_HUB_DATA="$DATA" bun nx run os-hub:trusted-catalog-bootstrap --packages "$packages" --outputStyle=stream > "$OUT/publish-$name.txt" 2>&1
  local rc=$?; echo "[w2-lanes] END publish $name rc=$rc wall=$(( $(date +%s) - s ))s $(date '+%F %T')"; return $rc
}
for phase in "${PHASES[@]}"; do
  [ "$phase" = "$from" ] && go=1; [ $go -eq 1 ] || continue
  case $phase in
    b-lanes) lanes b "${B[@]}" || lanes b "${B[@]}" || exit 1 ;;
    b-publish) publish w2-catalog-b2 "stdio,gis,note,animate,block,writer,draw,puzzle,wfc" || exit 1 ;;
    rest-lanes) lanes rest "${REST[@]}" || lanes rest "${REST[@]}" || exit 1 ;;
    all-publish) publish w2-catalog-all all || exit 1 ;;
  esac
done
echo "[w2-lanes] DONE $(date '+%F %T')"
