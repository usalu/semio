#!/bin/zsh
# 🔁 Restage the generation3d react guest for lane `selection-prune-interact`, retrying through peer
# churn in the shared cargo tree (a peer's in-flight refactor breaks the workspace for minutes at a time).
LOG="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️09/PROCEDURAL-3D-END-TO-END/🗑️generated/react-interact/restage.txt"
DONE="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️09/PROCEDURAL-3D-END-TO-END/🗑️generated/react-interact/restage.done"
cd /Users/ueli/Documents/semio
rm -f "$DONE"
export CARGO_PROFILE_WASM_DEV_DEBUG=false NX_DAEMON=false
for attempt in $(seq 1 8); do
  date >> "$LOG"; echo "attempt=$attempt" >> "$LOG"
  bun nx run @semio-tech/framework-os-dev:activate-generation3d-react-dev >> "$LOG" 2>&1; rc=$?
  echo "EXIT=$rc attempt=$attempt" >> "$LOG"
  [ $rc -eq 0 ] && break
  sleep 180
done
date >> "$LOG"
touch "$DONE"
