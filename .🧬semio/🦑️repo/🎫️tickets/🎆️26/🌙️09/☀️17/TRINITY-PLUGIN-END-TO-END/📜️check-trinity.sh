#!/bin/zsh
# 🔱️ Native check of the trinity plugin crate (ticket 26/09/17/TRINITY-PLUGIN-END-TO-END).
cd /Users/ueli/Documents/semio || exit 1
LOG="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️17/TRINITY-PLUGIN-END-TO-END/🗑️generated/check-trinity.txt"
echo "start $(date -u +%FT%TZ)" > "$LOG"
cargo check -p semio-s-plugin-trinity --lib --message-format short >> "$LOG" 2>&1
echo "exit=$? $(date -u +%FT%TZ)" >> "$LOG"
