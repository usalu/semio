#!/bin/zsh
# 🛰️ Detached fem react dev serve for runtime verification: `serve-fem-react.sh fem2d 6086` / `fem3d 6087`.
VARIANT="${1:-fem2d}"; PORT="${2:-6086}"
LOG="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️06/FEM-PLUGIN-END-TO-END/🗑️generated/serve-$VARIANT-react.txt"
exec > "$LOG" 2>&1
cd "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript"
export NX_DAEMON=false SEMIO_RENDERER=react SEMIO_VITE_HMR=0 S_OS_PORT="$PORT" SEMIO_PLUGIN="$VARIANT"
date
bun ./📜️script.ts serve "$VARIANT" react dev
echo "EXIT=$?"
date
