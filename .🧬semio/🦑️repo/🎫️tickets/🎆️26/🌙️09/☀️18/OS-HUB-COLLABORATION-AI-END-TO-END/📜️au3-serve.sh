#!/bin/zsh
# 🛰️ Detached React dev serve pointed at a live hub (ticket 26/09/18 slice AU3).
# Usage: 📜️au3-serve.sh <variant> <uiPort> <hubOrigin>
VARIANT="$1"; PORT="$2"; HUB="$3"
LOG="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/OS-HUB-COLLABORATION-AI-END-TO-END/🗑️generated/au3-${VARIANT}-serve.txt"
exec > "$LOG" 2>&1
cd "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript" || exit 1
export NX_DAEMON=false SEMIO_RENDERER=react SEMIO_VITE_HMR=0 S_OS_PORT="$PORT" SEMIO_PLUGIN="$VARIANT" S_HUB_URL="$HUB"
date
bun ./📜️script.ts serve "$VARIANT" react dev
echo "EXIT=$?"
date
