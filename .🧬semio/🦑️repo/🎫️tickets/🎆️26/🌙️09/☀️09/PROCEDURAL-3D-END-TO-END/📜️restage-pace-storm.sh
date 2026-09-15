#!/bin/zsh
# 🔁 Retry the generation3d react restage for the tool-run-pace-storm lane until the shared tree compiles.
LOG="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️09/PROCEDURAL-3D-END-TO-END/🗑️generated/pace-storm/restage.txt"
cd /Users/ueli/Documents/semio
export CARGO_PROFILE_WASM_DEV_DEBUG=false NX_DAEMON=false
for attempt in $(seq 1 8); do
  date >> "$LOG"; echo "attempt=$attempt" >> "$LOG"
  bun nx run @semio-tech/framework-os-dev:activate-generation3d-react-dev >> "$LOG" 2>&1; rc=$?
  echo "EXIT=$rc attempt=$attempt" >> "$LOG"
  [ $rc -eq 0 ] && break
  sleep 240
done
date >> "$LOG"
echo "RESTAGE-DONE" >> "$LOG"
