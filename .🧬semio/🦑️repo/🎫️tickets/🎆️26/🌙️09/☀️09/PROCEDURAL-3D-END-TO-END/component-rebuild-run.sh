#!/bin/sh
# 🏗️ Sequential component rebuild after the escrow-slot reclaim fix; gated on memory/swap and other compiles.
cd /Users/ueli/Documents/semio || exit 1
TF=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️09/PROCEDURAL-3D-END-TO-END"
G="$TF/🗑️generated/component-rebuild"; mkdir -p "$G"
for name in "$@"; do
  while :; do
    pct=$(memory_pressure | tail -1 | sed 's/[^0-9]*\([0-9]*\)%.*/\1/'); sw=$(sysctl -n vm.swapusage | sed 's/.*used = \([0-9]*\).*/\1/'); dk=$(df -g .cache/b/r/m | awk 'NR==2{print $4}')
    [ "$pct" -ge 45 ] && [ "$sw" -lt 3500 ] && [ "$dk" -ge 9 ] && ! ps -eo command | grep -E "run [a-z@/-]+:(component|wasm)" | grep -v -E "grep|component-rebuild" | grep -q . && break; sleep 20
  done
  echo "$(date +%T) start $name" >> "$G/run.txt"
  repo run "$name" -c prototype --exclude-dependencies > "$G/$(echo $name | tr ':' '_').txt" 2>&1
  code=$?; echo "$(date +%T) end $name exit $code" >> "$G/run.txt"
  [ "$code" -eq 0 ] || break
done
