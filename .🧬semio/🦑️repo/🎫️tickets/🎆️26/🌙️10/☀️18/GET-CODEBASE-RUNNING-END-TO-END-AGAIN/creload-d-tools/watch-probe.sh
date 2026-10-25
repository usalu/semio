#!/bin/bash
# Ticket input script: waits for the dev site on port 6061, then runs probe.mjs once per delay given.
#   watch-probe.sh <out-prefix> <delayMs...>
here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
root=/Users/ueli/Documents/semio
out="$1"; shift
until curl -sf -o /dev/null http://127.0.0.1:6061/; do sleep 3; done
sleep 5
export PLAYWRIGHT_BROWSERS_PATH="$root/.🧬semio/🦑️repo/⚡️cache/tools/ms-playwright"
for d in "$@"; do
  nice -n 10 node "$here/probe.mjs" "$root/node_modules/playwright/.." "$d" > "$out-$d.log" 2>&1
done
