#!/bin/zsh
# 📋️ Restage the forms guest wasm + materialize the react dev module (ticket 26/09/16/FORMS-PLUGIN-END-TO-END).
cd /Users/ueli/Documents/semio || exit 1
export CARGO_PROFILE_WASM_DEV_DEBUG=false NX_DAEMON=false SEMIO_RENDERER=react S_OS_PORT=6058
LOG=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️16/FORMS-PLUGIN-END-TO-END/🗑️generated/activate-forms-react.txt"
echo "start $(date -u +%FT%TZ)" > "$LOG"
bun nx run @semio-tech/framework-os-dev:activate-forms-react-dev >> "$LOG" 2>&1
echo "exit=$? $(date -u +%FT%TZ)" >> "$LOG"
