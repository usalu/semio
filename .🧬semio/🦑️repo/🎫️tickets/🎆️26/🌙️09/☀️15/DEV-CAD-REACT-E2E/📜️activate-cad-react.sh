#!/bin/zsh
# 🔁 Restage the cad guest wasm into the served react dev plugin-modules tree.
LOG="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️15/DEV-CAD-REACT-E2E/🗑️generated/activate-cad-react.txt"
cd /Users/ueli/Documents/semio
export CARGO_PROFILE_WASM_DEV_DEBUG=false NX_DAEMON=false SEMIO_RENDERER=react CAD_JS_RENDERER_PLAY_PORT=6020
date >> "$LOG"
bun nx run @semio-tech/framework-os-dev:activate-cad-react-dev >> "$LOG" 2>&1; rc=$?
echo "EXIT=$rc" >> "$LOG"
date >> "$LOG"
ls -la "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/dist/dev/🔌️plugin-modules/📐️cad/" >> "$LOG" 2>&1
echo "ACTIVATE-DONE" >> "$LOG"
