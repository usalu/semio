#!/bin/zsh
# Detached s react-dev serve for S14. Usage: s14-serve.sh <port> <hubUrl>
PORT="${1:-6230}"; HUB="${2:-http://127.0.0.1:7730}"
LOG="/Users/ueli/Documents/semio/.tmp-ticket/wp-s14/generated/s14-serve-${PORT}.txt"
exec > "$LOG" 2>&1
cd "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript" || exit 1
export NX_DAEMON=false SEMIO_RENDERER=react SEMIO_VITE_HMR=0 SEMIO_PLUGIN=s S_OS_PORT="$PORT" S_HUB_URL="$HUB"
echo "PID=$$ PORT=$PORT HUB=$HUB $(date '+%F %T')"
bun ./📜️script.ts serve s react dev
echo "EXIT=$? $(date '+%F %T')"
