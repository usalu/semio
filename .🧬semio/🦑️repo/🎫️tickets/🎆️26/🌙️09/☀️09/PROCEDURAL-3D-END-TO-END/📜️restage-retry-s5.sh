#!/bin/zsh
# 🔁 Session 5: retry the generation3d react restage until the shared tree compiles (peer churn).
LOG="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️09/PROCEDURAL-3D-END-TO-END/🗑️generated/react-s5/restage.txt"
mkdir -p "$(dirname "$LOG")"
cd /Users/ueli/Documents/semio
export CARGO_PROFILE_WASM_DEV_DEBUG=false NX_DAEMON=false
for attempt in $(seq 1 8); do
  date >> "$LOG"; echo "attempt=$attempt" >> "$LOG"
  bun nx run @semio-tech/framework-os-dev:activate-generation3d-react-dev >> "$LOG" 2>&1; rc=$?
  echo "EXIT=$rc attempt=$attempt" >> "$LOG"
  [ $rc -eq 0 ] && break
  sleep 180
done
date >> "$LOG"
ls -la "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/dist/dev/🔌️plugin-modules/🌀️procedural/" | grep -E "wasm|descriptor" >> "$LOG"
echo "RESTAGE-S5-DONE" >> "$LOG"
