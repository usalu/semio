#!/bin/zsh
# 🛰️ Detached lowpoly react dev serve on 6078 for runtime verification.
LOG="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️29/LOWPOLY-END-TO-END-COMMANDS-IO-AND-MUTATIONS/🗑️generated/serve-lowpoly-react.txt"
exec > "$LOG" 2>&1
cd "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript"
export NX_DAEMON=false SEMIO_RENDERER=react SEMIO_VITE_HMR=0 S_OS_PORT=6078 SEMIO_PLUGIN=lowpoly
date
bun ./📜️script.ts serve lowpoly react dev
echo "EXIT=$?"
date
