#!/bin/zsh
# 🧪 Runs every fem3d test mentioning examples or set_active_example with the editor feature: `test-set-active-example.sh 1`.
LOG="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️16/FEM-3D-STACKED-CONCRETE-FOREST/🗑️generated/test-set-active-example-${1:-1}.txt"
cd /Users/ueli/Documents/semio
export RUST_MIN_STACK=134217728 CARGO_INCREMENTAL=0
date > "$LOG"
cargo test -p semio-s-artifact-fem-3d --features component-app-assembly --lib -- set_active_example example >> "$LOG" 2>&1
echo "EXIT=$?" >> "$LOG"
date >> "$LOG"
echo "TEST-DONE" >> "$LOG"
