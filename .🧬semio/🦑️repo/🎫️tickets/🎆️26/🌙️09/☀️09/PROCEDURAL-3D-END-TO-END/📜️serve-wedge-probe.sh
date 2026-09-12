#!/bin/zsh
# 🛰️ Detached instrumented generation3d react dev serve (port 6028) for wedge diagnosis.
# $1 = output log basename under 🗑️generated/vite-wedge/ (default probe-6028)
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️09/PROCEDURAL-3D-END-TO-END"
exec > "$T/🗑️generated/vite-wedge/${1:-probe-6028}.txt" 2>&1
cd "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript"
export SEMIO_WEDGE_OWNWATCH="${SEMIO_WEDGE_OWNWATCH:-0}" SEMIO_WEDGE_KQUEUE="${SEMIO_WEDGE_KQUEUE:-0}" NX_DAEMON=false SEMIO_RENDERER=react SEMIO_PLUGIN=generation3d SEMIO_BUILD_MODE=dev SEMIO_VITE_HMR=0 S_OS_PORT=6028
date
bun "$T/🐍️vite-wedge-probe.ts"
echo "EXIT=$?"
