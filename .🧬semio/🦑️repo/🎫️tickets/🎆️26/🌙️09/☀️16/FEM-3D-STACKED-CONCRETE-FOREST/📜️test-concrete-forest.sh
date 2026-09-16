#!/bin/zsh
# 🧪 Runs the fem3d `concrete-forest` example tests (engine solve included), logged per run number: `test-concrete-forest.sh 1`.
LOG="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️16/FEM-3D-STACKED-CONCRETE-FOREST/🗑️generated/test-concrete-forest-${1:-1}.txt"
cd /Users/ueli/Documents/semio
export RUST_MIN_STACK=134217728 CARGO_INCREMENTAL=0
date > "$LOG"
cargo test -p semio-s-artifact-fem-3d --features component-app-assembly --lib -- examples::concrete_forest --nocapture >> "$LOG" 2>&1
echo "EXIT=$?" >> "$LOG"
date >> "$LOG"
echo "TEST-DONE" >> "$LOG"
