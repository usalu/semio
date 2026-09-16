#!/bin/zsh
# 🛂️ Re-emit the energy owner-root descriptor (`🔣️.json` + `🛂️.descriptor.semio`) from a fresh wasm32-wasip2 component.
cd "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🔋️energy/📦️packages/🦀️rust" || exit 1
export CARGO_PROFILE_WASM_DEV_DEBUG=false NX_DAEMON=false
LOG="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️06/ENERGY-PLUGIN-END-TO-END/🗑️generated/describe-energy.txt"
echo "start $(date -u +%FT%TZ)" > "$LOG"
bun ./📜️script.ts describe >> "$LOG" 2>&1
echo "exit=$? $(date -u +%FT%TZ)" >> "$LOG"
