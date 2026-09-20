#!/bin/zsh
# 🛰️ Detached react dev serve for a batch-A dormant plugin (ticket 26/09/18/OS-HUB-COLLABORATION-AI-END-TO-END).
# Usage: 📜️b1a-serve.sh <variant> <port>
VARIANT="$1"; PORT="$2"
LOG="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/OS-HUB-COLLABORATION-AI-END-TO-END/🗑️generated/f3-${VARIANT}-serve.txt"
exec > "$LOG" 2>&1
cd "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript" || exit 1
export NX_DAEMON=false SEMIO_RENDERER=react SEMIO_VITE_HMR=0 S_OS_PORT="$PORT" SEMIO_PLUGIN="$VARIANT"
date
bun ./📜️script.ts serve "$VARIANT" react dev
echo "EXIT=$?"
date
