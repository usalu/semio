#!/bin/sh
cd /Users/ueli/Documents/semio
. ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/asm-g-tools/env.sh"
F2=$(cat /private/tmp/asm-g/touch2.txt)
rm -rf /private/tmp/asm-g/store
RUNS=1 TOUCH="$F2" RSS_CAP_MB=5500 nice -n 10 /Users/ueli/.bun/bin/bun "$T/delta-build.mjs" "$O/final-inc" "@semio-tech/print-rs" test > "$O/final-inc.log" 2>&1
rm -rf /private/tmp/asm-g/store
FORCE_ALL=1 RUNS=1 TOUCH="$F2" RSS_CAP_MB=5500 nice -n 10 /Users/ueli/.bun/bin/bun "$T/delta-build.mjs" "$O/final-all" "@semio-tech/print-rs" test > "$O/final-all.log" 2>&1
nice -n 10 /Users/ueli/.bun/bin/bun "$T/cmp-gens.mjs" "$O/final-inc" "$O/final-all" > "$O/final-cmp.log" 2>&1
