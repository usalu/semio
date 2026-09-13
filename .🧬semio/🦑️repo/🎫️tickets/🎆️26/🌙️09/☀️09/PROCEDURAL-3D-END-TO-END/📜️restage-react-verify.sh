#!/bin/zsh
# 🔁 react-end-to-end-verification lane restage: the generation3d guest only (no flow_core change in
# this lane's fix), retried once on peer churn. Log: 🗑️generated/react-verify/restage.txt
LOG="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️09/PROCEDURAL-3D-END-TO-END/🗑️generated/react-verify/restage.txt"
cd /Users/ueli/Documents/semio
export CARGO_PROFILE_WASM_DEV_DEBUG=false NX_DAEMON=false CARGO_INCREMENTAL=0
for attempt in 1 2 3; do
  date >> "$LOG"; echo "attempt=$attempt activate" >> "$LOG"
  bun nx run @semio-tech/framework-os-dev:activate-generation3d-react-dev >> "$LOG" 2>&1; rc=$?
  echo "EXIT=$rc attempt=$attempt" >> "$LOG"
  [ $rc -eq 0 ] && break
  sleep 120
done
echo "RESTAGE-DONE" >> "$LOG"; date >> "$LOG"
