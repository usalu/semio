#!/bin/zsh
# 🛰️ G6 detached note react dev serve on 6390 (local-only) for live-agent-loop-check.
LOG="/Users/ueli/Documents/semio/.tmp-ticket/wp-g6/generated/g6-serve-note-6390.txt"
exec > "$LOG" 2>&1
cd "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript"
export NX_DAEMON=false SEMIO_RENDERER=react SEMIO_VITE_HMR=0 S_OS_PORT=6390 SEMIO_PLUGIN=note ${G6_HUB_URL:+S_HUB_URL=$G6_HUB_URL}
[ -z "$G6_HUB_URL" ] && export S_LOCAL_ONLY=1
date
bun ./📜️script.ts serve note react dev
echo "EXIT=$?"
date
