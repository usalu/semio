#!/bin/zsh
# 🧹 Coordinator disk watchdog: below 40 GiB free, prune cargo incremental sessions idle > 90 min (regenerable, never a live session).
cd /Users/ueli/Documents/semio || exit 1
BUILD=".🧬semio/🦑️repo/⚡️cache/cargo/build"
while true; do
  free=$(df -g /System/Volumes/Data | tail -1 | awk '{print $4}')
  if [ "$free" -lt 40 ]; then
    for inc in "$BUILD/debug/incremental" "$BUILD/wasm32-wasip2/wasm-dev/incremental" "$BUILD/wasm-dev/incremental"; do
      [ -d "$inc" ] && find "$inc" -mindepth 2 -maxdepth 2 -type d -mmin +90 -exec rm -rf {} + 2>/dev/null
    done
    mid=$(df -g /System/Volumes/Data | tail -1 | awk '{print $4}')
    if [ "$mid" -lt 40 ]; then
      find ".🧬semio/🦑️repo/⚡️cache/nx" -mindepth 1 -maxdepth 1 -mmin +480 -exec rm -rf {} + 2>/dev/null
      find "$BUILD/wasm32-wasip2/debug/build" -mindepth 2 -maxdepth 2 -type d -mmin +1080 -exec rm -rf {} + 2>/dev/null
    fi
    low=$(df -g /System/Volumes/Data | tail -1 | awk '{print $4}')
    if [ "$low" -lt 30 ]; then
      find "$BUILD/debug/build" -mindepth 2 -maxdepth 2 -type d -mmin +720 -exec rm -rf {} + 2>/dev/null
      find "$BUILD/wasm32-unknown-unknown/debug/build" -mindepth 2 -maxdepth 2 -type d -mmin +720 -exec rm -rf {} + 2>/dev/null
    fi
    after=$(df -g /System/Volumes/Data | tail -1 | awk '{print $4}')
    echo "$(date '+%H:%M') pruned: ${free} -> ${after} GiB free"
    if [ "$after" -lt 25 ]; then echo "$(date '+%H:%M') CRITICAL: ${after} GiB free after prune"; fi
  fi
  sleep 180
done
