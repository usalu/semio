#!/bin/zsh
# 🛰️ Detached energy react dev serve on 6106 for runtime verification (launch entry `energy-react-attach` opens it).
LOG="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️16/ENERGY-3D-MODEL-TREE-INSPECTOR/🗑️generated/serve-energy-react.txt"
exec > "$LOG" 2>&1
cd "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript"
export NX_DAEMON=false SEMIO_RENDERER=react SEMIO_VITE_HMR=0 S_OS_PORT=6106 SEMIO_PLUGIN=energy
date
bun ./📜️script.ts serve energy react dev
echo "EXIT=$?"
date
