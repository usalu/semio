#!/bin/sh
cd /Users/ueli/Documents/semio
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️05/DRAW-PLUGIN-END-TO-END"
export PLAYWRIGHT_BROWSERS_PATH="$PWD/.🧬semio/🦑️repo/⚡️cache/tools/ms-playwright"
# usage: matrix.sh tag panels nonote count
for i in $(seq 1 "$4"); do
  rm -rf "$T/🗑️generated/m-$1-$i"
  NONOTE=$3 PANELS="$2" repo tools exec --light bun -- "$T/probe-blank-switch.mjs" --serve http://127.0.0.1:6064/ --locale en --output "$T/🗑️generated/m-$1-$i" > "$T/🗑️generated/m-$1-$i.log" 2>&1
  echo "$1 $i $(grep -h 'OK blank\|STALL' "$T/🗑️generated/m-$1-$i.log")"
done
