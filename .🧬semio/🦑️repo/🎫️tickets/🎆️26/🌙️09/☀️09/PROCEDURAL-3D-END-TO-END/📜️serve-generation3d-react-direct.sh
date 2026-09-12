#!/bin/zsh
# 🛰️ Direct vite serve for the procedural 3d react editor playground (port 6018), bypassing `📜️script.ts serve`
# when a peer's in-flight taxonomy/registry change breaks the script's registry load. Run under `screen -dmS g3dreact`.
LOG="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️09/PROCEDURAL-3D-END-TO-END/🗑️generated/s13-serve-react-direct.txt"
exec > "$LOG" 2>&1
cd "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript"
export NX_DAEMON=false
export S_OS_PORT=6018
export SEMIO_PLUGIN=generation3d SEMIO_RENDERER=react SEMIO_BUILD_MODE=dev SEMIO_VITE_HMR=0
export VITE_SEMIO_PLUGIN=generation3d VITE_SEMIO_RENDERER=react
date
bun /Users/ueli/Documents/semio/node_modules/vite/bin/vite.js --configLoader bundle --config ⚙️vite.config.ts --host 127.0.0.1 --port 6018 --strictPort
echo "EXIT=$?"
date
