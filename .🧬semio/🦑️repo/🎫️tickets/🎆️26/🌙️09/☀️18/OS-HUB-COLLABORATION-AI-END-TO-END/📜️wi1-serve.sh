#!/bin/zsh
# 🛰️ Detached react dev serve of the ALREADY-STAGED `s` variant for slice WI1's live inference run.
# Serve only — no re-activation (the staged tree under ⚡️cache/vite/os-dev/s-react-dev survives).
# Usage: 📜️wi1-serve.sh <variant> <port>
VARIANT="$1"; PORT="$2"
TICKET="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/OS-HUB-COLLABORATION-AI-END-TO-END"
LOG="$TICKET/🗑️generated/wi1-serve-${VARIANT}-${PORT}.txt"
exec > "$LOG" 2>&1
cd "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript" || exit 1
export NX_DAEMON=false SEMIO_RENDERER=react SEMIO_VITE_HMR=0 S_OS_PORT="$PORT" SEMIO_PLUGIN="$VARIANT"
date
echo "WI1-SERVE variant=$VARIANT port=$PORT"
bun ./📜️script.ts serve "$VARIANT" react dev
echo "EXIT=$?"
date
