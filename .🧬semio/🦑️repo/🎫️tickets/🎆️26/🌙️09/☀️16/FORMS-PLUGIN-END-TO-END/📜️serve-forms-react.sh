#!/bin/zsh
# 🛰️ Detached forms react dev serve on 6058 for runtime verification.
LOG="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️16/FORMS-PLUGIN-END-TO-END/🗑️generated/serve-forms-react.txt"
exec > "$LOG" 2>&1
cd "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript"
export NX_DAEMON=false SEMIO_RENDERER=react SEMIO_VITE_HMR=0 S_OS_PORT=6058 SEMIO_PLUGIN=forms
date
bun ./📜️script.ts serve forms react dev
echo "EXIT=$?"
date
