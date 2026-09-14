#!/bin/zsh
# 🔬️ Run the react battery's `interact` lane alone against port 6025 (lane selection-prune-interact).
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️09/PROCEDURAL-3D-END-TO-END"
TAG="${1:-before}"
cd "$T"
export SEMIO_BATTERY_URL=http://127.0.0.1:6025/?plugin=generation3d
export SEMIO_BATTERY_ROOT=react-interact
rm -f "$T/🗑️generated/react-interact/interact-$TAG.done"
bun 🐍️react-battery.mjs --only=interact > "$T/🗑️generated/react-interact/battery-$TAG.txt" 2>&1
echo "EXIT=$?" >> "$T/🗑️generated/react-interact/battery-$TAG.txt"
cp -f "$T/🗑️generated/react-interact/interact/results.json" "$T/🗑️generated/react-interact/results-$TAG.json" 2>/dev/null
touch "$T/🗑️generated/react-interact/interact-$TAG.done"
