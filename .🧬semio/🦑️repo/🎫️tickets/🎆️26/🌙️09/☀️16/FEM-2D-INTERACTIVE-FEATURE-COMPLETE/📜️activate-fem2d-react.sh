#!/bin/zsh
# 🔁 Build + materialize the fem guest wasm and activate the fem2d react dev runtime (log per run number): `activate-fem2d-react.sh 1`.
LOG="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️16/FEM-2D-INTERACTIVE-FEATURE-COMPLETE/🗑️generated/activate-fem2d-react-${1:-1}.txt"
cd /Users/ueli/Documents/semio
export CARGO_PROFILE_WASM_DEV_DEBUG=false NX_DAEMON=false SEMIO_RENDERER=react SEMIO_BUILD_BUDGET_MS=3600000
date > "$LOG"
bun nx run "@semio-tech/framework-os-dev:activate-fem2d-react-dev" --skip-nx-cache >> "$LOG" 2>&1; echo "EXIT=$?" >> "$LOG"
ls -la "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/dist/dev/🔌️plugin-modules/🏗️fem/" >> "$LOG" 2>&1
date >> "$LOG"
echo "ACTIVATE-DONE" >> "$LOG"
