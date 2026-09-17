#!/bin/zsh
# 🧪 Type-check the remodeling artifact crate's test target (ticket 26/09/06/REMODEL-PLUGIN-END-TO-END).
cd /Users/ueli/Documents/semio || exit 1
LOG=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️06/REMODEL-PLUGIN-END-TO-END/🗑️generated/check-remodel-artifact-tests.txt"
echo "start $(date -u +%FT%TZ)" > "$LOG"
cargo check -p semio-s-artifact-remodel-remodeling --lib --tests -j 4 --message-format=short >> "$LOG" 2>&1
echo "exit=$? $(date -u +%FT%TZ)" >> "$LOG"
