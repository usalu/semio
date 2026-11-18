#!/bin/sh
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/FINISH-SHARED-BORDER-CONSOLIDATION-ACROSS-UI"
pkill -f "port 5397" 2>/dev/null; sleep 1
cd "/Users/ueli/Documents/semio/🏢️semio-tech/🎡️play"
SEMIO_BUILD_MODE=prototype SEMIO_VITE_CACHE_DIRECTORY=/Users/ueli/Documents/semio/.cache/b/p/m/border-play2 nohup /Users/ueli/Documents/semio/node_modules/.bin/vite --config "$T/vite-play-frozen.config.ts" --mode prototype --port 5397 --strictPort --host 127.0.0.1 > "$T/🗑️generated/play-dev.log" 2>&1 &
sleep 6
curl -s -o /dev/null -w "%{http_code}\n" http://127.0.0.1:5397/
