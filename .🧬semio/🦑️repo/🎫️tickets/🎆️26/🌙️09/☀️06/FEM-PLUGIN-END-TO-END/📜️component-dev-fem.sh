#!/bin/zsh
# 🏗️ Build the fem guest wasm component (wasm32-wasip2, wasm-dev profile) through Nx into the shared build-dir.
LOG="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️06/FEM-PLUGIN-END-TO-END/🗑️generated/component-dev-${1:-1}.txt"
cd /Users/ueli/Documents/semio
export CARGO_PROFILE_WASM_DEV_DEBUG=false NX_DAEMON=false SEMIO_BUILD_BUDGET_MS=3600000
date > "$LOG"
bun nx run @semio-tech/fem-plugin:component-dev --skip-nx-cache >> "$LOG" 2>&1; rc=$?
echo "EXIT=$rc" >> "$LOG"
date >> "$LOG"
echo "COMPONENT-DONE" >> "$LOG"
