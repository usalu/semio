#!/bin/zsh
# Usage: zsh serve.sh <variant> <port> <hubUrl> [dev|release] — one react serve bound to a hub (no local hub claim).
VARIANT="$1"; PORT="$2"; HUB="$3"; LANE="${4:-dev}"
cd /Users/ueli/Documents/semio/*framework/*products/*os/*modules/*dev/*packages/*typescript || exit 1
export NX_DAEMON=false SEMIO_RENDERER=react SEMIO_VITE_HMR=0 S_OS_PORT="$PORT" S_HUB_URL="$HUB" SEMIO_PLUGIN="$VARIANT" S_LOCAL_ONLY=1
echo "SERVE pid=$$ variant=$VARIANT lane=$LANE port=$PORT hub=$HUB $(date +%s)"
exec bun ./📜️script.ts serve "$VARIANT" react "$LANE"
