#!/usr/bin/env bash
set -u
T="$(cd "$(dirname "$0")" && pwd)"
pkill -f "vite-round3-elements" 2>/dev/null
sleep 2
mkdir -p /Users/ueli/Documents/semio/.cache/b/p/m/border-elements "$T/🗑️generated"
cd "$T/../../../../../../../🎓️teaching/🏛️architecture/📚️course" || exit 1
SEMIO_BUILD_MODE=prototype SEMIO_VITE_CACHE_DIRECTORY=/Users/ueli/Documents/semio/.cache/b/p/m/border-elements nohup /Users/ueli/Documents/semio/node_modules/.bin/vite --config "$T/vite-round3-elements.config.ts" --mode prototype --port 5442 --strictPort --host 127.0.0.1 > "$T/🗑️generated/vite-elements.log" 2>&1 &
sleep 6
curl -s -o /dev/null -m 60 -w "%{http_code}\n" http://127.0.0.1:5442/
