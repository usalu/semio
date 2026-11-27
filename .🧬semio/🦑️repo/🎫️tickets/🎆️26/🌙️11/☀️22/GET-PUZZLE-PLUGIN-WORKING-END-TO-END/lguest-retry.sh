#!/usr/bin/env bash
cd /home/deck/semio
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-PUZZLE-PLUGIN-WORKING-END-TO-END"; G="$T/🗑️generated"
export PUZZLE_COMPONENT_RELINK=1 PUZZLE_HUB_CODEGEN_UNITS=64
n=9
while :; do
  while [ -n "$(find 🧰️framework -name '*.rs' -mmin -4 2>/dev/null | head -1)" ]; do sleep 30; done
  log="$PWD/$G/lguest-deps-$n.log"; : > "$log"
  repo capacity run --memory=3500 --cpu=1 -- "$PWD/$T/lguest-deps.sh" "$log" > "$G/lguest-deps-$n.cap.log" 2>&1
  if grep -aq "dependency phase complete" "$log"; then echo "PHASE-A-DONE $n"; break; fi
  grep -aq "SIGTERM" "$log" || echo "[DEBUG] attempt $n failed: $(grep -a '^error' "$log" | head -1)"
  n=$((n+1)); sleep 60
done
