#!/bin/zsh
# usage: storybook.sh <scope> [secs]
scope=$1; secs=${2:-1500}; tag=${scope//\//-}
F="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
cache=/Users/ueli/Documents/semio/.cache/b/p/m/wbs-cache/storybook-$tag; mkdir -p $cache
log="$F/🗑️generated/wbs-storybook-$tag.log"
cd /Users/ueli/Documents/semio || exit 2
STORYBOOK_SCOPE=$scope SEMIO_BUILD_MODE=prototype SEMIO_VITE_CACHE_DIRECTORY=$cache SEMIO_STORYBOOK_BUILD=true STORYBOOK_DISABLE_TELEMETRY=1 nice -n 10 perl -e 'alarm shift; exec @ARGV' -- $secs \
  bun ${WBS_PRELOAD:+--preload $WBS_PRELOAD} node_modules/storybook/dist/bin/dispatcher.js build -c ${WBS_SBCONFIG:-.storybook} --output-dir /Users/ueli/.cache/wbs/out/storybook-$tag > "$log" 2>&1
echo "exit=$?" >> "$log"; tail -3 "$log"
