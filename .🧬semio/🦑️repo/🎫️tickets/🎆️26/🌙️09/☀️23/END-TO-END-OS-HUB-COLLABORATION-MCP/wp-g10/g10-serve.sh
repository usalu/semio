#!/bin/zsh
# 🛰️ G10 detached react dev serve of any variant. usage: g10-serve.sh <variant> <port> [hubOrigin]  (private agent-bridge rendezvous)
VARIANT="$1"; PORT="$2"; HUB="$3"
LOG="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s11-g10-logs/serve-$VARIANT-$PORT.txt"
exec > "$LOG" 2>&1
cd "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript" || exit 1
export NX_DAEMON=false SEMIO_RENDERER=react SEMIO_VITE_HMR=0 S_OS_PORT="$PORT" SEMIO_PLUGIN="$VARIANT" S_AGENT_BRIDGE_DIR=/Users/ueli/Documents/semio/.tmp-ticket/wp-g10/bridge S_AGENT_CREDENTIALS_DIR=/Users/ueli/Documents/semio/.tmp-ticket/wp-g10/credentials
if [ -n "$HUB" ]; then export S_HUB_URL="$HUB"; else export S_LOCAL_ONLY=1; fi
date
bun ./📜️script.ts serve "$VARIANT" react dev
echo "EXIT=$?"
date
