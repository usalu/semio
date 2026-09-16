#!/bin/zsh
# 🧪 Native cargo tests for the fem plugin crate and both artifact crates (log per run number).
LOG="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️06/FEM-PLUGIN-END-TO-END/🗑️generated/test-fem-native-${1:-1}.txt"
cd /Users/ueli/Documents/semio
export NX_DAEMON=false SEMIO_BUILD_BUDGET_MS=3600000
date > "$LOG"
for project in fem-plugin fem-2d-rs fem-3d-rs; do
  echo "=== ${project}:test" >> "$LOG"
  bun nx run "@semio-tech/${project}:test" --skip-nx-cache >> "$LOG" 2>&1; echo "EXIT[$project]=$?" >> "$LOG"
done
date >> "$LOG"
echo "TEST-DONE" >> "$LOG"
