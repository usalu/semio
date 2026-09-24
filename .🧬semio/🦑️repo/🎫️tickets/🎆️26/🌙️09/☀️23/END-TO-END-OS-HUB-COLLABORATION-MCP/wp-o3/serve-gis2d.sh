#!/bin/zsh
PORT="$1"; HUB="$2"
G=/Users/ueli/Documents/semio/.tmp-wp-o3/generated
cd /Users/ueli/Documents/semio/*framework/*products/*os/*modules/*dev/*packages/*typescript || exit 1
export NX_DAEMON=false SEMIO_RENDERER=react SEMIO_VITE_HMR=0 S_OS_PORT="$PORT" S_HUB_URL="$HUB" SEMIO_PLUGIN=gis2d
exec bun ./📜️script.ts serve gis2d react dev
