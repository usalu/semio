#!/bin/zsh
cd "/Users/ueli/Documents/semio/🧰️framework/🔨️modules"
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/FINISH-SHARED-BORDER-CONSOLIDATION-ACROSS-UI/🗑️generated"
for m in "$@"; do
  n=$(basename $(dirname $(dirname $(dirname $m))))
  timeout 1500 repo cargo --store=isolated --configuration=prototype -- test --offline --manifest-path "$m" --lib ${=EXTRA} > "$T/r3n-$n-$$.log" 2>&1
  echo "== $m" >> "$T/r3n-summary.log"; grep -E "^error|test result|FAILED|panicked|failed" "$T/r3n-$n-$$.log" | cut -c1-250 | head -25 >> "$T/r3n-summary.log"
done
echo ALLDONE >> "$T/r3n-summary.log"
