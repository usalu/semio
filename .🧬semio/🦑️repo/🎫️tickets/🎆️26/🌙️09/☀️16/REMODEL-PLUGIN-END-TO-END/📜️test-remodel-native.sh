#!/bin/zsh
# 🧪 Native unit tests of the remodeling artifact crate + plugin crate (ticket 26/09/16/REMODEL-PLUGIN-END-TO-END).
cd /Users/ueli/Documents/semio || exit 1
LOG=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️16/REMODEL-PLUGIN-END-TO-END/🗑️generated/test-remodel-native.txt"
echo "start $(date -u +%FT%TZ)" > "$LOG"
cargo test -p semio-s-artifact-remodel-remodeling --lib -j 6 -- --test-threads=4 >> "$LOG" 2>&1
echo "artifact-exit=$? $(date -u +%FT%TZ)" >> "$LOG"
cargo test -p semio-s-plugin-remodel --lib -j 6 >> "$LOG" 2>&1
echo "exit=$? $(date -u +%FT%TZ)" >> "$LOG"
