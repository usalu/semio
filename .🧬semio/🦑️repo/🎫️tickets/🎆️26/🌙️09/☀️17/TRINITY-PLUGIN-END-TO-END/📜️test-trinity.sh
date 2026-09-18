#!/bin/zsh
# 🧪️ Native unit tests for both trinity artifact crates (ticket 26/09/17/TRINITY-PLUGIN-END-TO-END).
cd /Users/ueli/Documents/semio || exit 1
LOG="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️17/TRINITY-PLUGIN-END-TO-END/🗑️generated/test-trinity.txt"
echo "start $(date -u +%FT%TZ)" > "$LOG"
RUST_MIN_STACK=33554432 cargo test -p semio-s-artifact-trinity-jack -p semio-s-artifact-trinity-rewriting --lib --features component-app-assembly --no-fail-fast -- --test-threads=2 >> "$LOG" 2>&1
echo "exit=$? $(date -u +%FT%TZ)" >> "$LOG"
