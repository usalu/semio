#!/bin/sh
# [graphlat-g] batch reuse-once: sh apply.sh apply|revert|status (14 anchored edits in 5 files + 4 new law directories). Apply AFTER plan-once.
cd /Users/ueli/Documents/semio || exit 1
HERE='.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/graphlat-g-tools/staged'
exec python3 "$HERE/batch_generic.py" "$HERE/reuse-once" "${1:-status}"
