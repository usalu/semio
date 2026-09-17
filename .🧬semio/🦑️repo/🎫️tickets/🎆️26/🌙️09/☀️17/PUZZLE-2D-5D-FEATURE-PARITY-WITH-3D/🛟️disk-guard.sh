#!/usr/bin/env bash
# 🛟️ Keeps the fleet alive: prunes unlocked cargo incremental sessions when free disk drops under a floor.
# Usage: nohup ./🛟️disk-guard.sh [floorGiB=30] [minutes=20] &
ROOT="/Users/ueli/Documents/semio"
PRUNE="$ROOT/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️06/PUZZLE-2D-END-TO-END/🧹️prune-incremental.py"
FLOOR="${1:-30}"; MINUTES="${2:-20}"
while true; do
  free=$(df -g / | tail -1 | awk '{print $4}')
  if [ "$free" -lt "$FLOOR" ]; then
    echo "$(date '+%H:%M:%S') free=${free}GiB < ${FLOOR}GiB → prune"; python3 "$PRUNE" --minutes "$MINUTES"
  fi
  sleep 240
done
