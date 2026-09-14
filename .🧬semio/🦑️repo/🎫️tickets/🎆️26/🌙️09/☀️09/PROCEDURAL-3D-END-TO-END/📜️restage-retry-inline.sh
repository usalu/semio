#!/bin/zsh
# 🔁 Lane flow-inline-continuation: retry the generation3d react+wgpu restage until the shared tree compiles.
LOG="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️09/PROCEDURAL-3D-END-TO-END/🗑️generated/flow-inline/restage.txt"
mkdir -p "$(dirname "$LOG")"
cd /Users/ueli/Documents/semio
export CARGO_PROFILE_WASM_DEV_DEBUG=false NX_DAEMON=false
for target in activate-generation3d-react-dev activate-generation3d-wgpu-dev; do
  for attempt in $(seq 1 8); do
    date >> "$LOG"; echo "target=$target attempt=$attempt" >> "$LOG"
    bun nx run "@semio-tech/framework-os-dev:$target" >> "$LOG" 2>&1; rc=$?
    echo "EXIT=$rc target=$target attempt=$attempt" >> "$LOG"
    [ $rc -eq 0 ] && break
    sleep 120
  done
done
date >> "$LOG"
echo "RESTAGE-INLINE-DONE" >> "$LOG"
