#!/bin/sh
cd /Users/ueli/Documents/semio
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️05/DRAW-PLUGIN-END-TO-END"
export PLAYWRIGHT_BROWSERS_PATH="$PWD/.🧬semio/🦑️repo/⚡️cache/tools/ms-playwright"
for i in $(seq 1 "${2:-6}"); do
  rm -rf "$T/🗑️generated/u-$1-$i"
  PROBE_DIAG= repo tools exec --light bun -- "$T/probe-blank-switch.mjs" --serve http://127.0.0.1:6064/ --locale en --output "$T/🗑️generated/u-$1-$i" > "$T/🗑️generated/u-$1-$i.log" 2>&1
  echo "$1 $i $(grep -h 'OK blank\|STALL\|UNSTUCK\|nudge' "$T/🗑️generated/u-$1-$i.log" | tr '\n' ' ')"
  grep -q STALL "$T/🗑️generated/u-$1-$i.log" && break
done
