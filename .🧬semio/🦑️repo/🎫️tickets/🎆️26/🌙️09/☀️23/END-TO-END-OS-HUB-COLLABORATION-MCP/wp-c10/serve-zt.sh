#!/bin/zsh
# Usage: zsh serve-zt.sh <port> <hubUrl> <dataDir> [profile] — one `s` react dev serve joining a local hub through its session broker.
PORT="$1"; HUB="$2"; DATA="$3"; PROFILE="${4:-developer}"
cd /Users/ueli/Documents/semio/*framework/*products/*os/*modules/*dev/*packages/*typescript || exit 1
export NX_DAEMON=false SEMIO_RENDERER=react SEMIO_VITE_HMR=0 S_OS_PORT="$PORT" S_HUB_URL="$HUB" SEMIO_PLUGIN=s OS_HUB_DATA="$DATA" SEMIO_DEV_LOCAL_HUB_PROFILE="$PROFILE"
echo "SERVE pid=$$ port=$PORT hub=$HUB data=$DATA profile=$PROFILE $(date +%s)"
exec bun ./📜️script.ts serve s react dev
