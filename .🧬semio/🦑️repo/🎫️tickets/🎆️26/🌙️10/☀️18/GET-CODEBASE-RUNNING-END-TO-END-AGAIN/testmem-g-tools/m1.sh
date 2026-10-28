#!/bin/sh
# m1: peak tree RSS + wall of dashboard TS suites under `bun test`, default vs --smol (graph-free, direct)
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
O="$C/🗑️generated/testmem-g"
D="🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧪️tests"
for f in "🎛️dispatch" "🌐️language" "🏗️ci"; do
 for mode in plain smol; do
  if [ "$mode" = smol ]; then cmd="bun --smol test --timeout 120000 ./$D/$f/🟦️.ts"; else cmd="bun test --timeout 120000 ./$D/$f/🟦️.ts"; fi
  python3 "$C/testmem-g-tools/tree-rss.py" "m1-$mode-$f" "$O/m1-$mode-$f.json" -- $cmd >> "$O/m1.jsonl" 2>&1
 done
done
