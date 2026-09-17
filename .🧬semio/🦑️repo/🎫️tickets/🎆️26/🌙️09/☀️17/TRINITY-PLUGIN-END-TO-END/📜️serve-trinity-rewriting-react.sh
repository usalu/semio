#!/bin/zsh
# 🛰️ Detached trinity rewriting react dev serve on 6056 for runtime verification.
LOG="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️17/TRINITY-PLUGIN-END-TO-END/🗑️generated/serve-trinity-rewriting-react.txt"
exec > "$LOG" 2>&1
cd "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript"
export NX_DAEMON=false SEMIO_RENDERER=react SEMIO_VITE_HMR=0 S_OS_PORT=6056 SEMIO_PLUGIN=trinity-rewriting
date
bun ./📜️script.ts serve trinity-rewriting react dev
echo "EXIT=$?"
date
