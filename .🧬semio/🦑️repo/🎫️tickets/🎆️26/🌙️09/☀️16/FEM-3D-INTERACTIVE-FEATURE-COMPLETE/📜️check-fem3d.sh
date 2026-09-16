#!/bin/zsh
# 🧪 Native cargo check of the fem3d artifact crate (lib + tests) with the editor feature, logged per run number: `check-fem3d.sh 1`.
LOG="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️16/FEM-3D-INTERACTIVE-FEATURE-COMPLETE/🗑️generated/check-fem3d-${1:-1}.txt"
cd /Users/ueli/Documents/semio
export CARGO_INCREMENTAL=0
date > "$LOG"
cargo check -p semio-s-artifact-fem-3d --features component-app-assembly --tests --message-format short >> "$LOG" 2>&1
echo "EXIT=$?" >> "$LOG"
date >> "$LOG"
echo "CHECK-DONE" >> "$LOG"
