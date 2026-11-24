#!/bin/sh
# 🏗️ Stages the procedural component, then the flow hub, one at a time (gated on memory pressure and load); retries on a transiently unparseable graph.
cd /Users/ueli/Documents/semio || exit 1
TF=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️09/PROCEDURAL-3D-END-TO-END"
LOG="$TF/🗑️generated/lazy-activation-components.txt"
for name in semio-hub-procedural-component semio-hub-flow-component; do
  out="$TF/🗑️generated/lazy-activation-$name.txt"
  while :; do
    while :; do
      pct=$(memory_pressure | tail -1 | sed 's/[^0-9]*\([0-9]*\)%.*/\1/'); load=$(uptime | sed 's/.*load averages*: *\([0-9]*\).*/\1/')
      [ "$pct" -ge 35 ] && [ "$load" -lt 16 ] && break
      sleep 10
    done
    echo "$(date +%T) start $name" >> "$LOG"
    repo run "$name:component" -c prototype --exclude-dependencies > "$out" 2>&1
    code=$?
    echo "$(date +%T) end $name exit $code" >> "$LOG"
    if [ "$code" -ne 0 ] && grep -q -E "BUILD_SOURCE_UNPARSEABLE|Expected .\) but found|Descriptor emitter prerequisite build failed" "$out"; then sleep 120; continue; fi
    break
  done
done
