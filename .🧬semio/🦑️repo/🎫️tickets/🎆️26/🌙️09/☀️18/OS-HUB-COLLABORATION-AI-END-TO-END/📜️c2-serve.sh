#!/bin/zsh
# 🛰️ Detached react dev serve of an ALREADY-STAGED variant bound to a hub origin (slice C2).
# Usage: 📜️c2-serve.sh <variant> <port> <hubUrl>
VARIANT="$1"; PORT="$2"; HUB="$3"
TICKET="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/OS-HUB-COLLABORATION-AI-END-TO-END"
LOG="$TICKET/🗑️generated/c2-serve-${VARIANT}-${PORT}.txt"
exec > "$LOG" 2>&1
cd "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript" || exit 1
export NX_DAEMON=false SEMIO_RENDERER=react SEMIO_VITE_HMR=0 S_OS_PORT="$PORT" S_HUB_URL="$HUB" SEMIO_PLUGIN="$VARIANT"
date
echo "C2-SERVE variant=$VARIANT port=$PORT hub=$HUB"
bun ./📜️script.ts serve "$VARIANT" react dev
echo "EXIT=$?"
date
