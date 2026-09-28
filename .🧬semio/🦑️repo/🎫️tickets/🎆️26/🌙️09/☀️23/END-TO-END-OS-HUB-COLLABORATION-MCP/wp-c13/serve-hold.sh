#!/bin/zsh
# 🛎️ C13 (14b): holds S18's shared dev serve (`ensureDevServe` via os-dev `serve-hold`) bound to a hub. usage: zsh serve-hold.sh <port> [hubUrl]
cd /Users/ueli/Documents/semio/*framework/*products/*os/*modules/*dev/*packages/*typescript || exit 1
export NX_DAEMON=false
echo "SERVE-HOLD pid=$$ port=$1 hub=$2 $(date '+%F %T')"
exec nice -n 10 bun ./📜️script.ts serve-hold --serve "http://127.0.0.1:$1/" ${2:+--hub} ${2:+$2}
