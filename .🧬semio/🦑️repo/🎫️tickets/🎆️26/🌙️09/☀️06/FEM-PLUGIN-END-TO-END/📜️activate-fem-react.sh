#!/bin/zsh
# 🔁 Build + materialize the fem guest wasm and activate the fem2d/fem3d react dev runtimes (log per run number).
LOG="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️06/FEM-PLUGIN-END-TO-END/🗑️generated/activate-fem-react-${1:-1}.txt"
cd /Users/ueli/Documents/semio
export CARGO_PROFILE_WASM_DEV_DEBUG=false NX_DAEMON=false SEMIO_RENDERER=react SEMIO_BUILD_BUDGET_MS=3600000
date > "$LOG"
for variant in fem2d fem3d; do
  echo "=== activate-$variant-react-dev" >> "$LOG"
  bun nx run "@semio-tech/framework-os-dev:activate-$variant-react-dev" >> "$LOG" 2>&1; echo "EXIT[$variant]=$?" >> "$LOG"
done
ls -la "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/dist/dev/🔌️plugin-modules/🏗️fem/" >> "$LOG" 2>&1
date >> "$LOG"
echo "ACTIVATE-DONE" >> "$LOG"
