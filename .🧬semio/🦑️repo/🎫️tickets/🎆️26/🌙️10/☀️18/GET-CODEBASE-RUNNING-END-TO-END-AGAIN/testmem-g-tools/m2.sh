#!/bin/sh
# m2: peak tree RSS + wall per `bun test` file (sampling 0.1 s) over a fixed list of harmless suites -> per-worker size distribution of the bun-test runner
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
O="$C/🗑️generated/testmem-g"
P="🧰️framework/🔨️modules/🏃️process"; L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"; D="🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧪️tests"
i=0
for f in "$P/🏗️build/🌡️host-capacity/🧪️tests/🟦️.ts" "$P/🏗️build/🌡️host-capacity/🧪️tests/📐️estimates/🟦️.ts" "$P/🏗️build/🌡️host-capacity/🧪️tests/🚪️gate/🟦️.ts" "$P/🏗️build/🌡️host-capacity/🧪️tests/📡️sampling/🟦️.ts" "$P/🧪️testing/🗂️suites/🧪️tests/🟦️.ts" "$P/🧪️testing/🧪️vitest/🧪️tests/🟦️.ts" "$P/🧪️testing/🎚️budget/🧪️tests/🟦️.ts" "$L/🏗️build/🌡️host-capacity/🧪️tests/🟦️.ts" "$L/🏗️build/🌡️host-capacity/🧪️tests/🧮️arguments/🟦️.ts" "$L/🏃️process/📋️context/🔣️policy/🧪️tests/🟦️.ts" "$D/⌨️controls/🟦️.ts" "$D/♻️reuse/🟦️.ts" "$D/🔎️source-discovery/🟦️.ts" "$D/🪟️windows/🟦️.ts" "$D/🌀️control-plane/🟦️.ts" "$D/🎛️dispatch/🟦️.ts" "$D/🏗️ci/🟦️.ts"; do
 i=$((i+1))
 TREE_RSS_INTERVAL=0.1 python3 "$C/testmem-g-tools/tree-rss.py" "m2-$i $f" "$O/m2-$i.json" -- bun test --timeout 120000 "./$f" >> "$O/m2.jsonl" 2>&1
done
