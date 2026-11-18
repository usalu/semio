#!/bin/sh
R=/Users/ueli/Documents/semio
T="$R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/FINISH-SHARED-BORDER-CONSOLIDATION-ACROSS-UI"
name=$1; port=$2; cwd=$3; cfg=$4
mkdir -p "$R/.cache/b/p/m/r3-$name"
cd "$cwd" || exit 1
NODE_OPTIONS=--experimental-transform-types SEMIO_BUILD_MODE=prototype SEMIO_VITE_CACHE_DIRECTORY="$R/.cache/b/p/m/r3-$name" nohup "$R/node_modules/.bin/vite" --configLoader native --config "$cfg" --mode prototype --port "$port" --strictPort --host 127.0.0.1 > "$T/🗑️generated/r3-$name.log" 2>&1 &
echo $! >> "$T/🗑️generated/r3-pids.txt"
