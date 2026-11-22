#!/usr/bin/env bash
set -euo pipefail
R=/Users/ueli/Documents/semio
BUN=$R/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
D="$R/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript"
VARIANT=${1:-s}; PORT=${2:-6070}
CFG="✏️s/🧑‍💻dev/🎭️variants/🌍️gis/🏗️builder/🌐️vite/🟦️.ts"
[ "$VARIANT" = "s" ] || CFG="$R/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🏗️builder/🌐️vite/🟦️.ts"
cd "$D"
export SEMIO_PLUGIN=$VARIANT SEMIO_RENDERER=react SEMIO_BUILD_MODE=prototype SEMIO_BRAND= VITE_SEMIO_PLUGIN=$VARIANT VITE_SEMIO_RENDERER=react VITE_SEMIO_APP_ID= S_OS_PORT=$PORT S_LOCAL_ONLY=1 S_HUB_URL= SEMIO_VITE_HMR=0
export SEMIO_VITE_CACHE_DIRECTORY="$R/.cache/b/p/m/vite/state/$VARIANT-react-prototype"
mkdir -p "$SEMIO_VITE_CACHE_DIRECTORY"
exec "$BUN" "$R/node_modules/vite/bin/vite.js" --mode prototype --config "$R/$CFG" --configLoader native --host 127.0.0.1 --port "$PORT" --strictPort
