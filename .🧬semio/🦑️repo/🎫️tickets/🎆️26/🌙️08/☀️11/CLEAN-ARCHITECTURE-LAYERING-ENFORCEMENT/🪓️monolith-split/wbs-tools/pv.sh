#!/bin/zsh
# usage: pv.sh <name> <root> <config> <port>  : vite preview of /Users/ueli/.cache/wbs/out/<name>; prints '/' status and asset check, then stops
name=$1; root=$2; cfg=$3; port=$4; prefix=${5:-}
F="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
cache=/Users/ueli/Documents/semio/.cache/b/p/m/wbs-cache/$name; mkdir -p $cache
log="$F/🗑️generated/wbs-preview-$name.log"
cd "$root" || exit 2
SEMIO_BUILD_MODE=prototype SEMIO_VITE_CACHE_DIRECTORY=$cache nice -n 10 bun /Users/ueli/Documents/semio/node_modules/vite/bin/vite.js preview --mode prototype --config "$cfg" --configLoader runner --outDir /Users/ueli/.cache/wbs/out/$name --host 127.0.0.1 --port $port --strictPort > "$log" 2>&1 &
pid=$!
for i in {1..60}; do curl -s -o /dev/null "http://127.0.0.1:$port/" && break; sleep 1; done
node "$F/wbs-tools/fetch-check.mjs" "http://127.0.0.1:$port$prefix" "/Users/ueli/.cache/wbs/out/$name" | tee -a "$log"
kill $pid 2>/dev/null; sleep 1; kill -9 $pid 2>/dev/null; echo stopped
