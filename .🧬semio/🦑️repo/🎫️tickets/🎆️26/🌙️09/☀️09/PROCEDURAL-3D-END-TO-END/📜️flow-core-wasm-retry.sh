#!/bin/zsh
# 🔁 Retry the flow CORE browser wasm build until the shared tree compiles (peer churn in
# `🔌️plugin/⚛️reactor/🔄️turn`), then report whether `fixtureChanged` reached the served binary.
# `flow_core_bg.wasm` is what the React node-graph session runs — `🕸️wasm` is cfg'd OUT of the
# wasm32-wasip2 plugin components, so NO plugin restage rebuilds it.
LOG="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️09/PROCEDURAL-3D-END-TO-END/🗑️generated/react-gen-wire/flow-core-retry.txt"
WASM="/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🫀️core/🕸️bindings/flow_core_bg.wasm"
cd /Users/ueli/Documents/semio
export NX_DAEMON=false CARGO_INCREMENTAL=0
for attempt in $(seq 1 20); do
  date >> "$LOG"; echo "attempt=$attempt" >> "$LOG"
  bun nx run semio-framework-os-flow-core:wasm >> "$LOG" 2>&1; rc=$?
  echo "EXIT=$rc attempt=$attempt" >> "$LOG"
  if [ $rc -eq 0 ]; then break; fi
  sleep 120
done
ls -la "$WASM" >> "$LOG"
echo "fixtureChanged=$(strings -a "$WASM" | grep -c fixtureChanged)" >> "$LOG"
echo "RETRY-DONE" >> "$LOG"
