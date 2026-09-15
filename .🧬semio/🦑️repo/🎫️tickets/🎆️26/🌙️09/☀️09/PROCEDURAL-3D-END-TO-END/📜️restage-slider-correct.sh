#!/bin/zsh
# 🔁 Restage the generation3d React guest for lane `slider-reevaluation-correctness`, then republish
# the brep extension — this lane changed the brep kernel (`validate_gate_sync`, `compact_unreachable`),
# which the extension links, so a restage alone would serve the old gate. Waits for any peer restage of
# the same target to finish first (two concurrent nx runs of one target contend on the same lock), then
# retries through peer compile churn.
LOG="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️09/PROCEDURAL-3D-END-TO-END/🗑️generated/slider-correct/restage.txt"
cd /Users/ueli/Documents/semio
export CARGO_PROFILE_WASM_DEV_DEBUG=false NX_DAEMON=false NX_SKIP_NX_CACHE=true
for _ in $(seq 1 120); do
  pgrep -f "activate-generation3d-react-dev" > /dev/null || break
  sleep 30
done
for attempt in $(seq 1 8); do
  date >> "$LOG"; echo "attempt=$attempt" >> "$LOG"
  bun nx run @semio-tech/framework-os-dev:activate-generation3d-react-dev >> "$LOG" 2>&1; rc=$?
  echo "STAGE-EXIT=$rc attempt=$attempt" >> "$LOG"
  bunx nx run @semio-tech/framework-os-dev:plugin --args="flow-extension-brep" >> "$LOG" 2>&1; prc=$?
  echo "PLUGIN-EXIT=$prc attempt=$attempt" >> "$LOG"
  if [ $rc -eq 0 ] && [ $prc -eq 0 ]; then break; fi
  sleep 180
done
date >> "$LOG"
echo "RESTAGE-DONE" >> "$LOG"
