#!/bin/sh
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
O="$C/🗑️generated/testmem-g"
export PLAYWRIGHT_BROWSERS_PATH="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/tools/ms-playwright"
for n in 0 1 2 3; do
 if [ "$n" = 0 ]; then cmd="node -e 0"; else cmd="node $C/testmem-g-tools/pw-browsers.mjs $n"; fi
 TREE_RSS_INTERVAL=0.25 python3 "$C/testmem-g-tools/tree-rss.py" "m5-chromium-$n" "$O/m5-$n.json" -- $cmd >> "$O/m5.jsonl" 2>&1
done
