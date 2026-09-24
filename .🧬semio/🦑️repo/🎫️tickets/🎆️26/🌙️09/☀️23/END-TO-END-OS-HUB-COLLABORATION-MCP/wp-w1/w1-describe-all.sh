#!/bin/zsh
# 🛂️ W1: describe every plugin/extension component through its own Nx `describe` target, one fleet wasm-mutex
# hold per component, shared cargo target (no CARGO_TARGET_DIR override: consumers read the shared wasm-dev).
# usage: zsh w1-describe-all.sh [project …]   (default: every project with a describe target, wfc+note first)
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
echo "[w1-describe] ${#PROJECTS[@]} projects $(date '+%F %T')"
for p in "${PROJECTS[@]}"; do
  n=${p#@semio-tech/}
  if [ -f "$OUT/describe-$n.ok" ]; then echo "[w1-describe] SKIP $n"; continue; fi
  echo "[w1-describe] START $n $(date '+%F %T')"
  s=$(date +%s)
  zsh "$MUTEX[1]" wasm w1 -- bun nx run "$p:describe" --outputStyle=stream > "$OUT/describe-$n.txt" 2>&1
  rc=$?
  echo "[w1-describe] END $n rc=$rc wall=$(( $(date +%s) - s ))s $(date '+%F %T')"
  [ $rc -eq 0 ] && touch "$OUT/describe-$n.ok" || tail -15 "$OUT/describe-$n.txt" | sed 's/^/    /'
done
echo "[w1-describe] DONE $(date '+%F %T')"
