#!/bin/zsh
# 🛰️ Detached procedural 3d react dev serve on port 6048 for the node-graph camera/label lane.
# A SECOND serve next to the shared 6018 one: this lane rebuilt `semio-framework-os-flow-core:wasm`,
# and vite never invalidates `node_modules/@semio-tech/flow-core`, so only a freshly started server
# hands the page the new guest. Run under `screen -dmS g3dfit <this script>` so it outlives the shell.
# Modelled on 📜️serve-generation3d-react.sh.
LOG="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️09/PROCEDURAL-3D-END-TO-END/🗑️generated/camera-fit/serve-6048.txt"
exec > "$LOG" 2>&1
cd "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript"
export NX_DAEMON=false SEMIO_RENDERER=react SEMIO_VITE_HMR=0 S_OS_PORT=6048
date
bun ./📜️script.ts serve generation3d react dev
echo "EXIT=$?"
date
