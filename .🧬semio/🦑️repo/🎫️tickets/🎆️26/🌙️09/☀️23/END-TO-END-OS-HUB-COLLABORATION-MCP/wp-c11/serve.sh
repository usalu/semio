#!/bin/zsh
# 🌐️ C11: one react `s` serve bound to a hub (join-only), dev or release lane. Usage: zsh serve.sh <variant> <port> <hubUrl> [dev|release]
VARIANT="$1"; PORT="$2"; HUB="$3"; LANE="${4:-dev}"
cd /Users/ueli/Documents/semio/*framework/*products/*os/*modules/*dev/*packages/*typescript || exit 1
export NX_DAEMON=false SEMIO_RENDERER=react SEMIO_VITE_HMR=0 S_OS_PORT="$PORT" S_HUB_URL="$HUB" SEMIO_PLUGIN="$VARIANT" S_LOCAL_ONLY=1
echo "SERVE pid=$$ variant=$VARIANT lane=$LANE port=$PORT hub=$HUB $(date +%s)"
exec nice -n 10 bun ./📜️script.ts serve "$VARIANT" react "$LANE"
