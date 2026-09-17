#!/bin/zsh
# 📸 Native check of the remodel plugin (ticket 26/09/06/REMODEL-PLUGIN-END-TO-END).
cd /Users/ueli/Documents/semio || exit 1
LOG=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️06/REMODEL-PLUGIN-END-TO-END/🗑️generated/check-remodel-native.txt"
echo "start $(date -u +%FT%TZ)" > "$LOG"
cargo check -p semio-s-plugin-remodel --lib --tests --message-format=short >> "$LOG" 2>&1
echo "exit=$? $(date -u +%FT%TZ)" >> "$LOG"
