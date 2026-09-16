#!/bin/zsh
# 🧪 Nextest of the fem2d crate WITH the editor feature, optional test-name filter: `nextest-fem2d-editor.sh <run> [filter...]`.
RUN="${1:-1}"; shift
LOG="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️16/FEM-2D-INTERACTIVE-FEATURE-COMPLETE/🗑️generated/nextest-fem2d-editor-${RUN}.txt"
cd /Users/ueli/Documents/semio
export RUST_MIN_STACK=134217728
date > "$LOG"
cargo nextest run -p semio-s-artifact-fem-2d --features component-app-assembly --profile fundamental --no-fail-fast --status-level fail --final-status-level fail -- "$@" --skip quick:: --skip long:: --skip exhaustive:: >> "$LOG" 2>&1
echo "EXIT=$?" >> "$LOG"
date >> "$LOG"
echo "NEXTEST-DONE" >> "$LOG"
