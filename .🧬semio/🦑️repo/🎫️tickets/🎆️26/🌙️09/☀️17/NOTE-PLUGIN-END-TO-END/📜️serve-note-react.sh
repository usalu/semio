#!/bin/zsh
# 🛰️ Detached note react dev serve on 6080 for runtime verification.
LOG="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️17/NOTE-PLUGIN-END-TO-END/🗑️generated/serve-note-react.txt"
exec > "$LOG" 2>&1
cd "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript"
export NX_DAEMON=false SEMIO_RENDERER=react SEMIO_VITE_HMR=0 S_OS_PORT=6080 SEMIO_PLUGIN=note
date
bun ./📜️script.ts serve note react dev
echo "EXIT=$?"
date
