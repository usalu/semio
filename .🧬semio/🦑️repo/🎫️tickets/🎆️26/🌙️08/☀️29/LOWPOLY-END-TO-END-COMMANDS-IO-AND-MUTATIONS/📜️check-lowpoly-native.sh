#!/bin/zsh
# 🦀️ Native check of the lowpoly artifact crate (ticket 26/08/29/LOWPOLY-END-TO-END-COMMANDS-IO-AND-MUTATIONS).
cd /Users/ueli/Documents/semio || exit 1
LOG=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️29/LOWPOLY-END-TO-END-COMMANDS-IO-AND-MUTATIONS/🗑️generated/check-lowpoly-native.txt"
echo "start $(date -u +%FT%TZ)" > "$LOG"
cargo check -p semio-s-artifact-lowpoly-lowpoly --lib --message-format=short >> "$LOG" 2>&1
echo "exit=$? $(date -u +%FT%TZ)" >> "$LOG"
