#!/bin/zsh
# 🚦 Gate for lane hot-swap-board-remount: journey + interact + flow-window on the lane's own :6023.
TICKET="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️09/PROCEDURAL-3D-END-TO-END"
LOG="$TICKET/🗑️generated/react-hotswap-battery.txt"
mkdir -p "$TICKET/🗑️generated"
exec > "$LOG" 2>&1
cd "$TICKET"
date
SEMIO_BATTERY_URL=http://127.0.0.1:6023/?plugin=generation3d SEMIO_BATTERY_ROOT=react-hotswap bun 🐍️react-battery.mjs --only=journey,interact,flow-window
echo "EXIT=$?"
date
