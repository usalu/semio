#!/bin/zsh
# 📏️ Restage the lowpoly guest wasm + materialize the react dev module (ticket 26/08/29/LOWPOLY-END-TO-END-COMMANDS-IO-AND-MUTATIONS).
cd /Users/ueli/Documents/semio || exit 1
export CARGO_PROFILE_WASM_DEV_DEBUG=false NX_DAEMON=false SEMIO_RENDERER=react S_OS_PORT=6078
LOG=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️29/LOWPOLY-END-TO-END-COMMANDS-IO-AND-MUTATIONS/🗑️generated/activate-lowpoly-react.txt"
echo "start $(date -u +%FT%TZ)" > "$LOG"
bun nx run @semio-tech/framework-os-dev:activate-lowpoly-react-dev >> "$LOG" 2>&1
echo "exit=$? $(date -u +%FT%TZ)" >> "$LOG"
