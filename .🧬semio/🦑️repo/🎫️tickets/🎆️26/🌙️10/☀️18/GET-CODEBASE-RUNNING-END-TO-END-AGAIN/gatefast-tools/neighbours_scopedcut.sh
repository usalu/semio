#!/bin/sh
# ✂️ Neighbour suites of batch scopedcut (run in a root): sh neighbours_scopedcut.sh <root>
cd "$1" || exit 1
L="./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
bun test --timeout 120000 "$L/🕸️graph/🕰️generations/🧪️tests/🌱️unborn-root/🟦️.ts" "$L/🕸️graph/🕰️generations/🧪️tests/🏃️world/🟦️.ts" "$L/🕸️graph/🕰️generations/🧪️tests/🪞️plan-once/🟦️.ts" "$L/🧭️invocation/🕸️graph/🧠️memo/🧪️tests/✏️edit/🟦️.ts" "$L/🧭️invocation/🕸️graph/🧠️memo/🧪️tests/🎯️slice/🟦️.ts" "$L/🧭️invocation/🕸️graph/🧠️memo/🧪️tests/🔂️reseal/🟦️.ts" "$L/🧭️invocation/🕸️graph/🧠️memo/🧪️tests/🧳️carry/🟦️.ts" "$L/🕸️graph/🧪️tests/🕰️generation-protocol/🟦️.ts"
