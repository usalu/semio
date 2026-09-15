#!/bin/zsh
# 🔁 Lane `concurrent-patch-intake`: restage the generation3d React guest with the nx cache SKIPPED, so the
# reactor's per-surface turn-page deferral (`⚛️reactor/📨️pending/🦀️.rs`) reaches the served wasm.
LOG="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️09/PROCEDURAL-3D-END-TO-END/🗑️generated/intake-restage/restage.txt"
mkdir -p "$(dirname "$LOG")"
cd /Users/ueli/Documents/semio
export CARGO_PROFILE_WASM_DEV_DEBUG=false NX_DAEMON=false NX_SKIP_NX_CACHE=true
for attempt in $(seq 1 6); do
  date >> "$LOG"; echo "attempt=$attempt" >> "$LOG"
  bun nx run @semio-tech/framework-os-dev:activate-generation3d-react-dev >> "$LOG" 2>&1; rc=$?
  echo "EXIT=$rc attempt=$attempt" >> "$LOG"
  [ $rc -eq 0 ] && break
  sleep 120
done
date >> "$LOG"
ls -la "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/dist/dev/🔌️plugin-modules/🌀️procedural/" | grep -E "wasm|descriptor" >> "$LOG"
echo "RESTAGE-INTAKE-DONE" >> "$LOG"
