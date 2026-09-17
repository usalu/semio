#!/bin/zsh
# 🛰️ Detached trinity jack react dev serve on 6054 for runtime verification.
LOG="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️17/TRINITY-PLUGIN-END-TO-END/🗑️generated/serve-trinity-jack-react.txt"
exec > "$LOG" 2>&1
cd "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript"
export NX_DAEMON=false SEMIO_RENDERER=react SEMIO_VITE_HMR=0 S_OS_PORT=6054 SEMIO_PLUGIN=trinity-jack
date
bun ./📜️script.ts serve trinity-jack react dev
echo "EXIT=$?"
date
