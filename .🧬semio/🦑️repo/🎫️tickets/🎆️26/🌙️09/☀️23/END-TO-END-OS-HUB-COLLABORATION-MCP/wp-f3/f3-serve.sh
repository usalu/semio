#!/bin/zsh
# 🚀️ F3 — hub-bound `s` React dev serve on a slice port, detached (w2-detach), with a start stamp for the boot timeline.
# usage: zsh f3-serve.sh <port> <tag> [hubUrl]   — prints the serve pid; log .🧬semio/🌐hub/s14-f3-logs/serve-<port>-<tag>.txt
setopt no_bg_nice
PORT="${1:?port}"
TAG="${2:?tag}"
HUB="${3:-http://127.0.0.1:7800}"
REPO=/Users/ueli/Documents/semio
LOGS="$REPO/.🧬semio/🌐hub/s14-f3-logs"
DEV="$REPO/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript"
DETACH="$REPO/.tmp-ticket/wp-w2/w2-detach.py"
LOG="$LOGS/serve-$PORT-$TAG.txt"
if lsof -nP -iTCP:$PORT -sTCP:LISTEN >/dev/null 2>&1; then echo "port $PORT busy"; exit 1; fi
echo "[f3-serve] start $(python3 -c 'import time;print(int(time.time()*1000))')" >> "$LOG"
python3 "$DETACH" "$LOG" zsh -c "cd '$DEV' && exec env NX_DAEMON=false SEMIO_RENDERER=react SEMIO_VITE_HMR=0 S_OS_PORT=$PORT S_HUB_URL=$HUB SEMIO_PLUGIN=s S_LOCAL_ONLY=1 bun ./📜️script.ts serve s react dev"
