#!/bin/zsh
# 👁️ Detached procedural 3d react dev serve pinned to the VIEWER app (port 6019).
# The shared editor serve on 6018 stays untouched: same plugin+renderer, so Vite's
# `cacheDir`/dep-hash are identical and no re-optimization is triggered.
# Run under `screen -dmS semio-g3d-viewer`.
LOG="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️09/PROCEDURAL-3D-END-TO-END/🗑️generated/viewer-serve-react.txt"
exec > "$LOG" 2>&1
cd "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript"
export NX_DAEMON=false
export S_OS_PORT=6019
export SEMIO_PLUGIN=generation3d SEMIO_RENDERER=react SEMIO_BUILD_MODE=dev SEMIO_VITE_HMR=0
export VITE_SEMIO_PLUGIN=generation3d VITE_SEMIO_RENDERER=react
export VITE_SEMIO_APP_ID='s.procedural.generation3d@1/*#viewer'
export VITE_SEMIO_APP_ROLE=viewer
date
bun /Users/ueli/Documents/semio/node_modules/vite/bin/vite.js --configLoader bundle --config "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🏗️builder/🌐️vite/🟦️.ts" --host 127.0.0.1 --port 6019 --strictPort
echo "EXIT=$?"
date
