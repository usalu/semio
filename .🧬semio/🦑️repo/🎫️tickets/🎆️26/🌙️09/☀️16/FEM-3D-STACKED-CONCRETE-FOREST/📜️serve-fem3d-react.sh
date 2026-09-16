#!/bin/zsh
# 🛰️ Detached fem3d react dev serve on 6087 for runtime verification (launch entry `fem3d-react-attach` opens it).
LOG="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️16/FEM-3D-STACKED-CONCRETE-FOREST/🗑️generated/serve-fem3d-react.txt"
exec > "$LOG" 2>&1
cd "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript"
export NX_DAEMON=false SEMIO_RENDERER=react SEMIO_VITE_HMR=0 S_OS_PORT=6087 SEMIO_PLUGIN=fem3d
date
bun ./📜️script.ts serve fem3d react dev
echo "EXIT=$?"
date
