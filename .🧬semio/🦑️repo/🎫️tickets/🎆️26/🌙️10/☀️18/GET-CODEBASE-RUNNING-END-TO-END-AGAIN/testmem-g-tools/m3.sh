#!/bin/sh
# m3: heavy bun-test suites that read the real build-library closure
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
O="$C/🗑️generated/testmem-g"
i=0
for f in "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🗓️scheduling/🧪️tests/🟦️.ts" "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/⚡️cache-contracts/🟦️.ts" "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧭️script-inputs/🔀️router/🧪️tests/🟦️.ts"; do
 i=$((i+1))
 TREE_RSS_INTERVAL=0.1 python3 "$C/testmem-g-tools/tree-rss.py" "m3-$i $f" "$O/m3-$i.json" -- bun test --timeout 300000 "./$f" >> "$O/m3.jsonl" 2>&1
done
