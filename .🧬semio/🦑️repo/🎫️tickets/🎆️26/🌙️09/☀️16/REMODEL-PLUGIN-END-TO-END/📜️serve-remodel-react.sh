#!/bin/zsh
# 🛰️ Detached remodel react dev serve on 6063 for runtime verification.
LOG="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️16/REMODEL-PLUGIN-END-TO-END/🗑️generated/serve-remodel-react.txt"
exec > "$LOG" 2>&1
cd "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript"
export NX_DAEMON=false SEMIO_RENDERER=react SEMIO_VITE_HMR=0 S_OS_PORT=6063 SEMIO_PLUGIN=remodel
date
bun ./📜️script.ts serve remodel react dev
echo "EXIT=$?"
date
