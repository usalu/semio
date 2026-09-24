#!/bin/zsh
VARIANT="$1"; PORT="$2"; HUB="$3"
cd /Users/ueli/Documents/semio/*framework/*products/*os/*modules/*dev/*packages/*typescript || exit 1
export NX_DAEMON=false SEMIO_RENDERER=react SEMIO_VITE_HMR=0 S_OS_PORT="$PORT" S_HUB_URL="$HUB" SEMIO_PLUGIN="$VARIANT"
exec bun ./📜️script.ts serve "$VARIANT" react dev
