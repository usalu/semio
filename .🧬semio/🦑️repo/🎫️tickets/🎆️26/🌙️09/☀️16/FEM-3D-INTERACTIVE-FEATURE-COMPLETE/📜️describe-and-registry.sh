#!/bin/zsh
# 🛂 Regenerate the fem descriptor pair (builds the wasm component) and the plugin registry's generated catalog: `describe-and-registry.sh 1`.
LOG="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️16/FEM-3D-INTERACTIVE-FEATURE-COMPLETE/🗑️generated/describe-and-registry-${1:-1}.txt"
cd /Users/ueli/Documents/semio
export CARGO_PROFILE_WASM_DEV_DEBUG=false NX_DAEMON=false SEMIO_BUILD_BUDGET_MS=3600000 CARGO_INCREMENTAL=0
date > "$LOG"
bun nx run @semio-tech/fem-plugin:describe --skip-nx-cache >> "$LOG" 2>&1; echo "DESCRIBE-EXIT=$?" >> "$LOG"
bun nx run @semio-tech/plugin-registry:generate --skip-nx-cache >> "$LOG" 2>&1; echo "REGISTRY-EXIT=$?" >> "$LOG"
date >> "$LOG"
echo "DESCRIBE-REGISTRY-DONE" >> "$LOG"
