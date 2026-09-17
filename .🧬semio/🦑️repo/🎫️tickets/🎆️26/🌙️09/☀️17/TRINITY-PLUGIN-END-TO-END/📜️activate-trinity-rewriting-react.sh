#!/bin/zsh
# 🔱️ Restage the trinity guest wasm + materialize the rewriting react dev module (ticket 26/09/17/TRINITY-PLUGIN-END-TO-END).
cd /Users/ueli/Documents/semio || exit 1
export CARGO_PROFILE_WASM_DEV_DEBUG=false NX_DAEMON=false SEMIO_RENDERER=react S_OS_PORT=6056
LOG="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️17/TRINITY-PLUGIN-END-TO-END/🗑️generated/activate-trinity-rewriting-react.txt"
echo "start $(date -u +%FT%TZ)" > "$LOG"
bun nx run @semio-tech/framework-os-dev:activate-trinity-rewriting-react-dev >> "$LOG" 2>&1
echo "exit=$? $(date -u +%FT%TZ)" >> "$LOG"
