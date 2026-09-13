#!/bin/zsh
# 🔁 Retry the generation3d wgpu activation until the shared plugin tree compiles again (peer churn); the 6118 serve picks the staged guest up on reload.
LOG="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️09/PROCEDURAL-3D-END-TO-END/🗑️generated/activate-wgpu-retry.txt"
cd /Users/ueli/Documents/semio
export CARGO_PROFILE_WASM_DEV_DEBUG=false NX_DAEMON=false
for attempt in $(seq 1 10); do
  date >> "$LOG"; echo "attempt=$attempt" >> "$LOG"
  bun nx run @semio-tech/framework-os-dev:activate-generation3d-wgpu-dev >> "$LOG" 2>&1; rc=$?
  echo "EXIT=$rc attempt=$attempt" >> "$LOG"
  [ $rc -eq 0 ] && break
  sleep 180
done
echo "RETRY-DONE" >> "$LOG"; date >> "$LOG"
