#!/bin/sh
# Stages the generation3d boot closure one exact name at a time, used components first (hub, brep, math), then playbook contributors, then the rest.
cd /Users/ueli/Documents/semio || exit 1
TF=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️09/PROCEDURAL-3D-END-TO-END"
LOG="$TF/🗑️generated/native-component-flow-staged.txt"
while pgrep -f "semio-s-plugin-flow-math-component:component" >/dev/null; do sleep 20; done
echo "$(date +%T) math run ended" >> "$LOG"
for name in semio-hub-flow-component semio-s-plugin-flow-brep-component semio-s-plugin-flow-math-component semio-s-plugin-playbook-procedural-component semio-hub-playbook-component semio-s-plugin-flow-list-component semio-s-plugin-flow-primitive-component semio-s-plugin-flow-text-component semio-s-plugin-flow-draw-component semio-s-plugin-flow-logic-component semio-s-plugin-flow-dictionary-component semio-s-plugin-flow-bim-component; do
  while :; do
    free=$(sysctl -n vm.swapusage | sed 's/.*free = \([0-9.]*\)M.*/\1/'); pct=$(memory_pressure | tail -1 | sed 's/[^0-9]*\([0-9]*\)%.*/\1/')
    [ "${free%.*}" -ge 1000 ] && [ "$pct" -ge 35 ] && break
    sleep 10
  done
  echo "$(date +%T) start $name" >> "$LOG"
  repo run "$name:component" -c prototype --exclude-dependencies > "$TF/🗑️generated/native-component-flow-staged-$name.txt" 2>&1
  echo "$(date +%T) end $name exit $?" >> "$LOG"
done
