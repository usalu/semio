#!/bin/zsh
# 🎨️ Restage the generation3d wgpu guest after the procedural rename fix-forward, so the wgpu stage is
# not left behind the React one. No wgpu battery is run here — the wgpu renderer is on hold.
DIR="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️09/PROCEDURAL-3D-END-TO-END/🗑️generated/rename-fixforward"
LOG="$DIR/restage-wgpu.txt"
mkdir -p "$DIR"
cd /Users/ueli/Documents/semio
export CARGO_PROFILE_WASM_DEV_DEBUG=false NX_DAEMON=false NX_SKIP_NX_CACHE=true
for attempt in 1 2 3; do
  date >> "$LOG"; echo "attempt=$attempt" >> "$LOG"
  bun nx run @semio-tech/framework-os-dev:activate-generation3d-wgpu-dev >> "$LOG" 2>&1; rc=$?
  echo "EXIT=$rc attempt=$attempt" >> "$LOG"
  [ $rc -eq 0 ] && break
  grep -aq "error\[E" "$LOG" && break
done
date >> "$LOG"
echo "RESTAGE-WGPU-FIXFORWARD-DONE rc=$rc" >> "$LOG"
