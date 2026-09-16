#!/bin/zsh
# 🛰️ Detached fem2d react dev serve on 6086 for runtime verification (launch entry `fem2d-react-attach` opens it).
LOG="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️16/FEM-2D-INTERACTIVE-FEATURE-COMPLETE/🗑️generated/serve-fem2d-react.txt"
exec > "$LOG" 2>&1
cd "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript"
export NX_DAEMON=false SEMIO_RENDERER=react SEMIO_VITE_HMR=0 S_OS_PORT=6086 SEMIO_PLUGIN=fem2d
date
bun ./📜️script.ts serve fem2d react dev
echo "EXIT=$?"
date
