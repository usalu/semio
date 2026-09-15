#!/bin/zsh
# 🔁 Restage the generation3d React guest for lane `react-closing-battery` (:6027), then republish the
# brep extension — the closing battery must read ONE stage that carries every lane that landed today,
# and `flow-extension-brep` links the kernel the boolean-input-lifetime lane changed, so a bare
# activate would serve the old operand copies. Waits for peer restages of the same target to drain
# first (two concurrent nx runs of one target contend on the same lock), then retries through peer
# compile churn. Never kills anything.
LOG="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️09/PROCEDURAL-3D-END-TO-END/🗑️generated/react-closing/restage.txt"
cd /Users/ueli/Documents/semio
export CARGO_PROFILE_WASM_DEV_DEBUG=false NX_DAEMON=false NX_SKIP_NX_CACHE=true
mkdir -p "$(dirname "$LOG")"
quiet=0
for _ in $(seq 1 12); do
  if pgrep -fl "activate-generation3d-react-dev" | grep -q "node /Users"; then quiet=0; else quiet=$((quiet + 1)); fi
  [ $quiet -ge 2 ] && break
  sleep 30
done
echo "peer-drain quiet=$quiet at $(date)" >> "$LOG"
for attempt in $(seq 1 6); do
  date >> "$LOG"; echo "attempt=$attempt" >> "$LOG"
  bun nx run @semio-tech/framework-os-dev:activate-generation3d-react-dev >> "$LOG" 2>&1; rc=$?
  echo "STAGE-EXIT=$rc attempt=$attempt" >> "$LOG"
  bunx nx run @semio-tech/framework-os-dev:plugin --args="flow-extension-brep" >> "$LOG" 2>&1; prc=$?
  echo "PLUGIN-EXIT=$prc attempt=$attempt" >> "$LOG"
  if [ $rc -eq 0 ] && [ $prc -eq 0 ]; then break; fi
  sleep 180
done
date >> "$LOG"
ls -la "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/dist/dev/🔌️plugin-modules/🌀️procedural/" | grep -aE "wasm|descriptor" >> "$LOG"
echo "RESTAGE-DONE" >> "$LOG"
