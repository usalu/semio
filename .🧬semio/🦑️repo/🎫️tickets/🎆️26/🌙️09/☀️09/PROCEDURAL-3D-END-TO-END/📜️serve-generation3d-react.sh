#!/bin/zsh
# 🛰️ Detached procedural 3d react dev serve (port 6018) for runtime verification.
# Run under `screen -dmS semio-g3d-react` so the serve outlives the agent shell.
LOG="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️09/PROCEDURAL-3D-END-TO-END/🗑️generated/s13-serve-react.txt"
exec > "$LOG" 2>&1
cd "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript"
export NX_DAEMON=false SEMIO_RENDERER=react SEMIO_VITE_HMR=0
date
bun ./📜️script.ts serve generation3d react dev
echo "EXIT=$?"
date
