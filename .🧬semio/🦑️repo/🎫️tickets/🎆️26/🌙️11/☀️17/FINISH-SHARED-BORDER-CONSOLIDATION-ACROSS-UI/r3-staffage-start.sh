#!/bin/sh
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/FINISH-SHARED-BORDER-CONSOLIDATION-ACROSS-UI"
pkill -f "port 5471" 2>/dev/null; sleep 1
cd "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🌐️site/🏗️builder/🌐️vite"
TEACHING_ARCHITECTURE_STAFFAGE_WATCH=off SEMIO_BUILD_MODE=prototype SEMIO_VITE_CACHE_DIRECTORY=/Users/ueli/Documents/semio/.cache/b/p/m/border-staffage-r3 nohup /Users/ueli/Documents/semio/node_modules/.bin/vite --config "$PWD/🟦️.ts" --mode prototype --port 5471 --strictPort --host 127.0.0.1 > "$T/🗑️generated/staffage-r3.log" 2>&1 &
sleep 5
curl -s -o /dev/null -w "%{http_code}\n" http://127.0.0.1:5471/
