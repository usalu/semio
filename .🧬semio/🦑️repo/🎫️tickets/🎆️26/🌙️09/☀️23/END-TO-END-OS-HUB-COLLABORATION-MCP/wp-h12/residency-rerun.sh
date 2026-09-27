#!/bin/zsh
# 🧠️ H12: residency-watch --rounds 2 on the running hub 8161 (B3 clone, 64 MiB) with the pair-content digest; capture → <capture>.
setopt no_bg_nice
cd "/Users/ueli/Documents/semio/🌎️hub/📦️packages/🟦️typescript" && bun ./📜️script.ts residency-watch --hub http://127.0.0.1:8161 --rounds 2 --settle-ms 30000 > "$1" 2>&1
echo "=== EXIT $? $(date +%T)" >> "$1"
