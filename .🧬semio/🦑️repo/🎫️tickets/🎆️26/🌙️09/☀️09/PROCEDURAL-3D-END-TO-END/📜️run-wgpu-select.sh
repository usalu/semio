#!/bin/zsh
# 🕹️ Lane `wgpu-selection-roundtrip`: gates on every other wgpu probe, then runs the two World3d
# battery lanes on 6118 into its own evidence root, so a peer run is never clobbered.
#
#   screen -dmS wgpusel "<ticket>/📜️run-wgpu-select.sh"
#
# The renderer wasm is NOT rebuilt: this lane's fix is the browser plugin bridge
# (`🎯️targets/🧊️wgpu/🐚️plugin-bridge/🟦️.ts`), bundled into `🎞️frame-worker/🤖️generated/🟨️.js` by
# `nx run @semio-tech/framework-renderer-wgpu:generate-frame-worker` and served straight off disk.
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️09/PROCEDURAL-3D-END-TO-END"
mkdir -p "$T/🗑️generated/wgpu-select"
LOG="$T/🗑️generated/wgpu-select/battery.txt"
exec > "$LOG" 2>&1
date
# 🚦️ Gates on the REAL probe processes only. A plain `pgrep -f` also matches every peer agent's own
# gate shell, whose command line quotes this very pattern — four of them were waiting on each other
# at 11:47 while no probe was running at all. Anchoring on `bun ` excludes a wrapper shell, and the
# bracket trick keeps this script's own poll from matching itself.
until [ -z "$(ps -eo command= | grep -aE '^bun [^ ]*wgpu-(batter[y]|[a-z0-9-]*-pro[b]e)')" ]; do sleep 20; done
echo "gate clear $(date)"
cd "$T"
SEMIO_BATTERY_ROOT=wgpu-select bun 🐍️wgpu-battery.mjs --only=world3d-editor,world3d-viewer
echo "BATTERY_EXIT=$?"
date
