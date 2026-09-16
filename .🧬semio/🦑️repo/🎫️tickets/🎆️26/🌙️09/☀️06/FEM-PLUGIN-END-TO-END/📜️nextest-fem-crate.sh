#!/bin/zsh
# 🧪 Full fundamental-level nextest run of one fem crate without fail-fast: `nextest-fem-crate.sh semio-s-artifact-fem-2d 1`.
CRATE="${1:-semio-s-artifact-fem-2d}"; RUN="${2:-1}"
LOG="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️06/FEM-PLUGIN-END-TO-END/🗑️generated/nextest-${CRATE}-${RUN}.txt"
cd /Users/ueli/Documents/semio
export RUST_MIN_STACK=134217728
date > "$LOG"
cargo nextest run -p "$CRATE" --profile fundamental --no-fail-fast --status-level fail --final-status-level fail -- --skip quick:: --skip long:: --skip exhaustive:: >> "$LOG" 2>&1
echo "EXIT=$?" >> "$LOG"
date >> "$LOG"
echo "NEXTEST-DONE" >> "$LOG"
