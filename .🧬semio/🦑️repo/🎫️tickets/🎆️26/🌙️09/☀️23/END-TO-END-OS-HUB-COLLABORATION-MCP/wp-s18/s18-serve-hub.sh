#!/bin/zsh
# 🛎️ S18: `serve s react dev` on <port> joined to <hub url> (HMR off), same env as ensureDevServe's devServeCommandV1.
D="/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript"
cd "$D" && export SEMIO_PLUGIN=s SEMIO_RENDERER=react SEMIO_VITE_HMR=0 S_OS_PORT="$1" S_HUB_URL="$2" NX_DAEMON=false && unset S_LOCAL_ONLY && exec bun "$D/📜️script.ts" serve s react dev
