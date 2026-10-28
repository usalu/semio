#!/bin/sh
# [graphlat-g] batch plan-once: python3 ../batch_generic.py <this dir> apply|revert|status (anchored edit of 🕰️generations/🟨️.mjs + the new law 🕰️generations/🧪️tests/🪞️plan-once)
cd /Users/ueli/Documents/semio || exit 1
HERE='.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/graphlat-g-tools/staged'
exec python3 "$HERE/batch_generic.py" "$HERE/plan-once" "${1:-status}"
