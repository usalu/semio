#!/bin/zsh
# WG8 session 12: one React `s` serve (dev lane) bound to a hub, for the cross-shell journey.
# Usage: zsh serve-react.sh <port> <hubUrl>   (launch detached: setopt no_bg_nice; nohup zsh serve-react.sh … > log 2>&1 & disown)
PORT="$1"; HUB="$2"
cd "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript" || exit 1
export NX_DAEMON=false SEMIO_RENDERER=react SEMIO_VITE_HMR=0 S_OS_PORT="$PORT" S_HUB_URL="$HUB" SEMIO_PLUGIN=s S_LOCAL_ONLY=1
echo "SERVE pid=$$ port=$PORT hub=$HUB $(date '+%F %T')"
exec bun ./📜️script.ts serve s react dev
