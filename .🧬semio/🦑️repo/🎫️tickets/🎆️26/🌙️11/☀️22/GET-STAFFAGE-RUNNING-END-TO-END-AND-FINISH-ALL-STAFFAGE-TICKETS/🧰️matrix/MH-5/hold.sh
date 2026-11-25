#!/bin/zsh
# 🪑️ Waits up to 170 s until a file exists and holds a line matching a pattern, then prints its lines without the queue chatter: hold.sh <file> <pattern> [tail lines]
timeout 170 zsh -c 'until grep -qE "$2" "$1" 2>/dev/null; do sleep 5; done' hold "$1" "$2"
if grep -qE "$2" "$1" 2>/dev/null; then grep -v "^\[capacity\]" "$1" | tail -${3:-30} | cut -c1-240; else echo "waiting ($(date +%H:%M)): $(tail -c 160 "$1" 2>/dev/null | tr '\n' ' ')"; fi
