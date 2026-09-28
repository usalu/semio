#!/bin/zsh
# 🖥️ S19 dev serve: `serve s react dev` on 6610, local-only, HMR off, NX daemon off. usage: zsh s19-serve.sh [port]
cd "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript" || exit 2
export SEMIO_PLUGIN=s SEMIO_RENDERER=react SEMIO_VITE_HMR=0 S_OS_PORT="${1:-6610}" S_LOCAL_ONLY=1 NX_DAEMON=false
unset S_HUB_URL
exec nice -n 5 bun ./📜️script.ts serve s react dev
