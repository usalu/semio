#!/bin/zsh
# 🛰️ G4 detached note react dev serve on 6330 (local-only) for live-agent-loop-check.
LOG="/Users/ueli/Documents/semio/.tmp-ticket/wp-g4/generated/g4-serve-note-6330.txt"
exec > "$LOG" 2>&1
cd "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript"
export NX_DAEMON=false SEMIO_RENDERER=react SEMIO_VITE_HMR=0 S_OS_PORT=6330 SEMIO_PLUGIN=note ${G4_HUB_URL:+S_HUB_URL=$G4_HUB_URL}
[ -z "$G4_HUB_URL" ] && export S_LOCAL_ONLY=1
date
bun ./📜️script.ts serve note react dev
echo "EXIT=$?"
date
