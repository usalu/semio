#!/bin/zsh
# 🔁 Coordinator React restage: flow_core wasm (the React flow session, not rebuilt by the plugin activation) then the generation3d guest activation; retries on peer churn.
LOG="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️09/PROCEDURAL-3D-END-TO-END/🗑️generated/restage-react-full.txt"
cd /Users/ueli/Documents/semio
export CARGO_PROFILE_WASM_DEV_DEBUG=false NX_DAEMON=false
for attempt in $(seq 1 6); do
  date >> "$LOG"; echo "attempt=$attempt flow-core" >> "$LOG"
  bunx nx run semio-framework-os-flow-core:wasm --skip-nx-cache >> "$LOG" 2>&1; echo "FLOWCORE-EXIT=$? attempt=$attempt" >> "$LOG"
  echo "attempt=$attempt activate" >> "$LOG"
  bun nx run @semio-tech/framework-os-dev:activate-generation3d-react-dev >> "$LOG" 2>&1; rc=$?
  echo "EXIT=$rc attempt=$attempt" >> "$LOG"
  [ $rc -eq 0 ] && break
  sleep 180
done
echo "RESTAGE-DONE" >> "$LOG"; date >> "$LOG"
