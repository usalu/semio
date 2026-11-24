#!/bin/sh
# Stages the flow components one exact name at a time (one graph read at a time, memory gate before each read).
cd /Users/ueli/Documents/semio || exit 1
TF=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️09/PROCEDURAL-3D-END-TO-END"
for name in semio-s-plugin-flow-math-component semio-s-plugin-flow-list-component semio-s-plugin-flow-logic-component semio-s-plugin-flow-text-component semio-s-plugin-flow-dictionary-component semio-s-plugin-flow-primitive-component semio-s-plugin-flow-draw-component semio-s-plugin-flow-brep-component semio-s-plugin-flow-bim-component semio-hub-flow-component; do
  while :; do
    free=$(sysctl -n vm.swapusage | sed 's/.*free = \([0-9.]*\)M.*/\1/'); pct=$(memory_pressure | tail -1 | sed 's/[^0-9]*\([0-9]*\)%.*/\1/')
    [ "${free%.*}" -ge 1500 ] && [ "$pct" -ge 40 ] && break
    sleep 10
  done
  echo "$(date +%T) start $name" >> "$TF/🗑️generated/native-component-flow-staged.txt"
  repo run "$name:component" -c prototype --exclude-dependencies > "$TF/🗑️generated/native-component-flow-staged-$name.txt" 2>&1
  echo "$(date +%T) end $name exit $?" >> "$TF/🗑️generated/native-component-flow-staged.txt"
done
