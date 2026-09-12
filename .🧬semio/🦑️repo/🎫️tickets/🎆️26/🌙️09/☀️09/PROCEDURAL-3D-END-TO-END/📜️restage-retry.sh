#!/bin/zsh
# 🔁 Retry the generation3d react restage until the shared tree compiles (peer churn), then republish flow-extension-brep.
LOG="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️09/PROCEDURAL-3D-END-TO-END/🗑️generated/restage-retry.txt"
cd /Users/ueli/Documents/semio
export CARGO_PROFILE_WASM_DEV_DEBUG=false NX_DAEMON=false
for attempt in $(seq 1 12); do
  date >> "$LOG"; echo "attempt=$attempt" >> "$LOG"
  bun nx run @semio-tech/framework-os-dev:activate-generation3d-react-dev >> "$LOG" 2>&1; rc=$?
  echo "EXIT=$rc attempt=$attempt" >> "$LOG"
  if [ $rc -eq 0 ]; then
    bunx nx run @semio-tech/framework-os-dev:plugin --args="flow-extension-brep" >> "$LOG" 2>&1; echo "PLUGIN-EXIT=$? attempt=$attempt" >> "$LOG"
    break
  fi
  sleep 300
done
date >> "$LOG"
ls -la "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/dist/dev/🔌️plugin-modules/🌀️procedural/" | grep -E "wasm|descriptor" >> "$LOG"
echo "RETRY-DONE" >> "$LOG"
