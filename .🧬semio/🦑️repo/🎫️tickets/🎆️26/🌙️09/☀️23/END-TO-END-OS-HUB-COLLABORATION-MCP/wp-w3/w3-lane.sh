#!/bin/zsh
# 🛤️ W3: one sequential build lane inside a chain's wasm hold. Each item is a plugin id (its `component-release`, which warms the
# shared wasm-release units the catalog bootstrap links) or `renderer` (the browser wgpu renderer `wasm-release`, never cut nor skipped). The
# lane stops before its next plugin item once <stop> exists; while a component-release runs, <stop>.pid names its nx process so the owner may cut it.
# usage: zsh w3-lane.sh <log prefix> <stop file> <item…>
setopt no_bg_nice
PREFIX="$1"; STOP="$2"; shift 2
for item in "$@"; do
  [ "$item" != renderer ] && [ -e "$STOP" ] && { echo "[lane] STOP before $item $(date '+%F %T')"; break; }
  echo "[lane] START $item $(date '+%F %T')"
  if [ "$item" = renderer ]; then
    bun nx run @semio-tech/framework-renderer-wgpu:wasm-release --skip-nx-cache --outputStyle=stream > "$PREFIX-renderer.txt" 2>&1
    rc=$?
  else
    bun nx run "@semio-tech/$item-plugin:component-release" --skip-nx-cache --outputStyle=stream > "$PREFIX-$item.txt" 2>&1 &
    echo $! > "$STOP.pid"
    wait $!
    rc=$?
    rm -f "$STOP.pid"
  fi
  echo "[lane] END $item rc=$rc $(date '+%F %T')"
done
echo "[lane] DONE $(date '+%F %T')"
