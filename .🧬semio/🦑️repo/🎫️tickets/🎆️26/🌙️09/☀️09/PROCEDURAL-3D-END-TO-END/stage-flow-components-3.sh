#!/bin/sh
# Stages the generation3d flow components one exact name at a time, hub first, gated on memory pressure and load; retries a component while the graph or a build source is transiently unparseable.
cd /Users/ueli/Documents/semio || exit 1
TF=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️09/PROCEDURAL-3D-END-TO-END"
LOG="$TF/🗑️generated/native-component-flow-staged.txt"
for name in semio-hub-flow-component semio-s-plugin-flow-brep-component semio-s-plugin-flow-math-component semio-s-plugin-flow-list-component semio-s-plugin-flow-primitive-component semio-s-plugin-flow-text-component semio-s-plugin-flow-draw-component semio-s-plugin-flow-logic-component semio-s-plugin-flow-dictionary-component semio-s-plugin-flow-bim-component; do
  out="$TF/🗑️generated/native-component-flow-staged-$name.txt"
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
