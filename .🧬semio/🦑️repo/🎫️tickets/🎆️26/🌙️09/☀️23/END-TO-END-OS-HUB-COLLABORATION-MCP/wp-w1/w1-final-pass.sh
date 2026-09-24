#!/bin/zsh
# 🏁️ W1 final pass after p4/p5 (every kind answers pack-schema-hash, structural hash landed). Each step is ONE fleet
# wasm-mutex hold running ONE product verb; Nx runs the per-component tasks inside it with --parallel=3 over the shared
# build-dir (fine-grain locking), so one hold covers the whole closure instead of 60 serial holds.
#   1 activate-s-react-dev: every component's component-dev → materialize-dev → session → activate
#   2 describe ×60: reuses the component-dev unit (pluginComponentRustcArgs + incremental=false) → committed descriptors
#   3 plugin-registry generate + check
#   4 component-release ×34 (warms the exact trusted-bootstrap units), os-hub build under the hub mutex,
#     trusted-catalog-bootstrap --packages all → .🧬semio/🌐hub/w1-catalog-b
# usage: zsh w1-final-pass.sh [from-step]
set -u
cd /Users/ueli/Documents/semio || exit 1
W=/Users/ueli/Documents/semio/.tmp-ticket/wp-w1
OUT=$W/generated
MUTEX=(/Users/ueli/Documents/semio/.tmp-ticket/*fleet-mutex.sh)
unset CARGO_TARGET_DIR CARGO_BUILD_TARGET_DIR
export CARGO_INCREMENTAL=0
from=${1:-1}
step() { echo "[w1-final] START $1 $(date '+%F %T')"; s=$(date +%s); }
done_() { echo "[w1-final] END $1 rc=$2 wall=$(( $(date +%s) - s ))s $(date '+%F %T')"; [ "$2" -eq 0 ] || { tail -40 "$3" | sed 's/^/    /'; exit "$2"; }; }
# 🔁️ Peers edit the shared tree while a 60-component closure builds; a transient compile break fails one task. Nx caches every
# task that succeeded, so a bounded retry after a pause re-runs only the broken ones against the repaired tree.
retry() { local name=$1 log=$2; shift 2; local rc=1; for attempt in 1 2 3 4; do zsh "$MUTEX[1]" wasm w1 -- "$@" > "$log" 2>&1; rc=$?; [ $rc -eq 0 ] && break; echo "[w1-final] RETRY $name attempt=$attempt rc=$rc $(date '+%F %T')"; grep -E 'error\[|error:' "$log" | head -3 | sed 's/^/      /'; cp "$log" "${log%.txt}-attempt$attempt.txt"; sleep 300; done; return $rc; }
DESCRIBE=$(bun nx show projects --with-target describe 2>/dev/null | python3 -c 'import json,sys;print(",".join(p for p in json.load(sys.stdin) if p!="@semio-tech/os-plugin-describe-rs"))')
RELEASE=$(echo stdio gis animate architect block cad dag demonstrator draw energy fem flow forms imperative layout lowpoly mathematical norm note playbook procedural process puzzle raster reasoning remodel sequence shooting sourcing space trinity vcs wfc writer | tr ' ' '\n' | sed 's|.*|@semio-tech/&-plugin|' | paste -sd, -)
if [ $from -le 1 ]; then
  step activate-s; retry activate-s "$OUT/final-1-activate.txt" bun nx run @semio-tech/framework-os-dev:activate-s-react-dev --parallel=3 --outputStyle=stream; done_ activate-s $? "$OUT/final-1-activate.txt"
fi
if [ $from -le 2 ]; then
  step describe-all; retry describe-all "$OUT/final-2-describe.txt" bun nx run-many -t describe --projects="$DESCRIBE" --parallel=3 --outputStyle=stream; done_ describe-all $? "$OUT/final-2-describe.txt"
fi
if [ $from -le 3 ]; then
  step generate; zsh "$MUTEX[1]" wasm w1 -- bun nx run @semio-tech/plugin-registry:generate --outputStyle=stream > "$OUT/final-3-generate.txt" 2>&1; done_ generate $? "$OUT/final-3-generate.txt"
  step check; bun nx run @semio-tech/plugin-registry:check --outputStyle=stream > "$OUT/final-3-check.txt" 2>&1; done_ check $? "$OUT/final-3-check.txt"
fi
if [ $from -le 4 ]; then
  step release-all; retry release-all "$OUT/final-4-release.txt" bun nx run-many -t component-release --projects="$RELEASE" --parallel=3 --outputStyle=stream; done_ release-all $? "$OUT/final-4-release.txt"
fi
if [ $from -le 5 ]; then
  cd $W && zsh w1-catalog.sh hub; cd /Users/ueli/Documents/semio
  step publish-all; cd $W && W1_PACKAGES=all W1_DATA_NAME=w1-catalog-b zsh w1-catalog.sh publish; rc=$?; cd /Users/ueli/Documents/semio; done_ publish-all $rc "$OUT/publish-w1-catalog-b.txt"
fi
echo "[w1-final] DONE $(date '+%F %T')"
