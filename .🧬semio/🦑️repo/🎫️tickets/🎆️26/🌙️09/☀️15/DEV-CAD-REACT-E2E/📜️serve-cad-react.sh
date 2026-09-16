#!/bin/zsh
# 🛰️ Detached cad react dev serve (port 6020) for runtime verification.
# Run under `screen -dmS semio-cad-react` so the serve outlives the agent shell.
LOG="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️15/DEV-CAD-REACT-E2E/🗑️generated/serve-cad-react.txt"
exec > "$LOG" 2>&1
cd "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript"
export NX_DAEMON=false SEMIO_RENDERER=react SEMIO_VITE_HMR=0 CAD_JS_RENDERER_PLAY_PORT=6020 S_OS_PORT=6020
date
bun ./📜️script.ts serve cad react dev
echo "EXIT=$?"
date
