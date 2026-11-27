#!/usr/bin/env bash
cd /home/deck/semio
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-PUZZLE-PLUGIN-WORKING-END-TO-END"; G="$T/🗑️generated"
export PUZZLE_COMPONENT_RELINK=1 PUZZLE_HUB_CODEGEN_UNITS=64
n=3
while :; do
  while [ -n "$(find 🧰️framework 🌎️hub -name '*.rs' -mmin -1 2>/dev/null | head -1)" ]; do sleep 30; done
  log="$G/lguest-final-$n.log"
  repo capacity run --memory=${LGUEST_MEM:-6500} --cpu=1 -- "$PWD/$T/lguest-build.sh" > "$log" 2>&1
  rc=$?
  echo "[DEBUG] attempt $n rc=$rc $(grep -a '^error' "$log" | head -1 | cut -c1-160)"
  [ $rc -eq 0 ] && { echo FINAL-DONE $n; break; }
  n=$((n+1)); sleep 60
done
