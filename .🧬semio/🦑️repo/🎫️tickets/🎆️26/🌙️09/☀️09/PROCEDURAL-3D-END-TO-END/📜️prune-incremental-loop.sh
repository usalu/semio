#!/bin/zsh
# 🧹 Fleet disk guard: every 120 s remove cargo incremental session dirs untouched for 10+ min under the shared build dir.
setopt nullglob
B="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build"
LOG="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️09/PROCEDURAL-3D-END-TO-END/🗑️generated/prune-incremental.txt"
while true; do
  n=0
  for d in $(find "$B" -maxdepth 3 -type d -name incremental 2>/dev/null); do
    for s in "$d"/*; do [ -d "$s" ] || continue; if [ -z "$(find "$s" -mmin -10 -print -quit 2>/dev/null)" ]; then rm -rf "$s" && n=$((n+1)); fi; done
  done
  echo "$(date '+%H:%M:%S') pruned=$n free=$(df -k / | tail -1 | awk '{printf "%.0fG",$4/1048576}')" >> "$LOG"
  sleep 120
done
