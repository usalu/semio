#!/bin/zsh
# 🛰️ Slice S12 — detached `s` react dev serve bound to a NAMED hub, so the hub-document journey runs
# against TC3e's three-package stdio/gis/note catalog on 7651 instead of S11's catalog-less 7641.
# Usage: 📜️s12-serve.sh <port> <hubUrl>
PORT="${1:-6072}"; HUB="${2:-http://127.0.0.1:7651}"
LOG="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/OS-HUB-COLLABORATION-AI-END-TO-END/🗑️generated/s12-serve-${PORT}.txt"
exec > "$LOG" 2>&1
cd "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript" || exit 1
export NX_DAEMON=false SEMIO_RENDERER=react SEMIO_VITE_HMR=0 SEMIO_PLUGIN=s S_OS_PORT="$PORT" S_HUB_URL="$HUB"
date
bun ./📜️script.ts serve s react dev
echo "EXIT=$?"
date
