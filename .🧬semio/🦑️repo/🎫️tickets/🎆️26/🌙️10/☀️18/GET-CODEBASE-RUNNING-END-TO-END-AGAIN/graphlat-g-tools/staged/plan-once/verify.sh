#!/bin/sh
# [graphlat-g] verify of batch plan-once. usage: sh verify.sh [live]   (default: before the batch landed, through the in-memory preload; "live": after apply)
cd /Users/ueli/Documents/semio || exit 1
C='.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/graphlat-g-tools/staged'
BUN=.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
G='🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph'
if [ "$1" = live ]; then PRE=""; LAW="$G/🕰️generations/🧪️tests/🪞️plan-once/🟦️.ts"; else sh "$C/plan-once/mirror-links.sh"; PRE="--preload $PWD/$C/preload_generic.ts"; LAW="$C/plan-once/tree/$G/🕰️generations/🧪️tests/🪞️plan-once/🟦️.ts"; export BATCH_DIRS="$PWD/$C/plan-once"; fi
for f in "$LAW" "$G/🕰️generations/🧪️tests/🟦️.ts" "$G/🧪️tests/🕰️generation-protocol/🟦️.ts" "$G/🕰️generations/🔭️coverage/🧪️tests/🟦️.ts" "$G/🕰️generations/🛌️yield/🧪️tests/🟦️.ts"; do
  echo "== $f" | cut -c1-40,80-200
  $BUN test --timeout 200000 $PRE "./$f" 2>&1 | grep -E "^\(fail\)|^ *[0-9]+ (pass|fail)"
done
