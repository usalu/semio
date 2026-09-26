#!/bin/zsh
# 🔥️ W2 (session 12): warms `component-release` units for the packages the running bootstrap reaches LAST, inside W2's own wasm hold
# (the bootstrap holds the fleet mutex): the bootstrap's fresh build is the same `cargo rustc … --profile wasm-release` unit, so a package
# warmed here is only uplifted when the bootstrap reaches it. Lanes walk the publication order from the END; the bootstrap walks it from the
# front. usage: zsh w2-warm-behind.sh <lane-label> <package…>
setopt no_bg_nice
set -u
cd /Users/ueli/Documents/semio || exit 1
OUT="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s12-w2-logs"
unset CARGO_TARGET_DIR CARGO_BUILD_TARGET_DIR
export CARGO_INCREMENTAL=0 NX_DAEMON=false
label=$1; shift
for p in "$@"; do
  s=$(date +%s); echo "[w2-warm $label] START $p $(date '+%T')"
  bun nx run @semio-tech/$p-plugin:component-release --outputStyle=stream > "$OUT/warm-$label-$p.txt" 2>&1
  echo "[w2-warm $label] END $p rc=$? wall=$(( $(date +%s) - s ))s $(date '+%T')"
done
echo "[w2-warm $label] DONE $(date '+%T')"
