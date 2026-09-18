#!/bin/zsh
# 📏️ Restage the layout guest wasm + materialize the react dev module (ticket 26/09/18/LAYOUT-PDF-EXPORT-END-TO-END).
cd /Users/ueli/Documents/semio || exit 1
export CARGO_PROFILE_WASM_DEV_DEBUG=false NX_DAEMON=false SEMIO_RENDERER=react S_OS_PORT=6079
LOG=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/LAYOUT-PDF-EXPORT-END-TO-END/🗑️generated/activate-layout-react.txt"
echo "start $(date -u +%FT%TZ)" > "$LOG"
bun nx run @semio-tech/framework-os-dev:activate-layout-react-dev >> "$LOG" 2>&1
echo "exit=$? $(date -u +%FT%TZ)" >> "$LOG"
