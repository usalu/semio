#!/bin/zsh
# 🧱️ W1: per component, in ONE fleet wasm-mutex hold: `describe` (links the component-dev unit into the shared
# target and writes the committed descriptor) then `materialize-dev` (its `component-dev` dependency reuses that
# same unit, then stages the browser module). Shared cargo target, CARGO_INCREMENTAL=0.
# Afterwards the caller runs plugin-registry generate/check and activate-s-react-dev.
# usage: zsh w1-stage-all.sh [project …]   (default: every project with a describe target, wfc+note first)
set -u
cd /Users/ueli/Documents/semio || exit 1
OUT=/Users/ueli/Documents/semio/.tmp-ticket/wp-w1/generated
MUTEX=(/Users/ueli/Documents/semio/.tmp-ticket/*fleet-mutex.sh)
unset CARGO_TARGET_DIR CARGO_BUILD_TARGET_DIR
export CARGO_INCREMENTAL=0
if [ $# -gt 0 ]; then
  PROJECTS=("$@")
else
  ALL=("${(@f)$(bun nx show projects --with-target describe 2>/dev/null | python3 -c 'import json,sys;[print(p) for p in json.load(sys.stdin) if p!="@semio-tech/os-plugin-describe-rs"]')}")
  PROJECTS=(@semio-tech/wfc-plugin @semio-tech/note-plugin ${ALL:#@semio-tech/(wfc|note)-plugin})
fi
echo "[w1-stage] ${#PROJECTS[@]} projects $(date '+%F %T')"
for p in "${PROJECTS[@]}"; do
  n=${p#@semio-tech/}
  if [ -f "$OUT/stage-$n.ok" ]; then echo "[w1-stage] SKIP $n"; continue; fi
  echo "[w1-stage] START $n $(date '+%F %T')"
  s=$(date +%s)
  zsh "$MUTEX[1]" wasm w1 -- zsh -c "bun nx run '$p:describe' --outputStyle=stream && bun nx run '$p:materialize-dev' --outputStyle=stream" > "$OUT/stage-$n.txt" 2>&1
  rc=$?
  echo "[w1-stage] END $n rc=$rc wall=$(( $(date +%s) - s ))s $(date '+%F %T')"
  [ $rc -eq 0 ] && touch "$OUT/stage-$n.ok" || tail -15 "$OUT/stage-$n.txt" | sed 's/^/    /'
done
echo "[w1-stage] DONE $(date '+%F %T')"
