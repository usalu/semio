#!/bin/zsh
# 🧪 The canonical nx test targets of the fem plugin (plugin crate, 2d artifact crate incl. TS/python contracts, JS package): `nx-tests.sh 1`.
LOG="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️16/FEM-2D-INTERACTIVE-FEATURE-COMPLETE/🗑️generated/nx-tests-${1:-1}.txt"
cd /Users/ueli/Documents/semio
export NX_DAEMON=false CARGO_INCREMENTAL=0 RUST_MIN_STACK=134217728 SEMIO_BUILD_BUDGET_MS=3600000
date > "$LOG"
for target in @semio-tech/fem-plugin:test @semio-tech/fem-2d-rs:test @semio-tech/fem-js:test; do
  echo "=== $target" >> "$LOG"
  bun nx run "$target" --skip-nx-cache >> "$LOG" 2>&1; echo "EXIT[$target]=$?" >> "$LOG"
done
date >> "$LOG"
echo "NX-TESTS-DONE" >> "$LOG"
