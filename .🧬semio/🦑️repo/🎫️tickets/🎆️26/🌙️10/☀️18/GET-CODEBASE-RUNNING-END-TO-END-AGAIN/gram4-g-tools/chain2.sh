#!/bin/sh
# [gram4-g] sequential after c1: row-site census (d0), JSC heap-size option (r2g = forceRAMSize 2 GB, the production graph child's minimum), second baseline (b1) and second collection-checkpoint run (g1), all root project only, S0.
. /private/tmp/gram4-g/env.sh
while pgrep -f "gram4-g-tools/full.mjs" > /dev/null; do sleep 10; done
O=$T/🗑️generated/gram4-g
sh $T/gram4-g-tools/run.sh d0 LIMIT=1 PROBES=0 ROW_SITES_OUT=/private/tmp/gram4-g/row-sites.json BATCHES=$S0,$T/gram4-g-tools/exp/row-sites > $O/d0.out 2>&1
sh $T/gram4-g-tools/run.sh r2g LIMIT=1 PROBES=0 BUN_JSC_forceRAMSize=2147483648 BATCHES=$S0 > $O/r2g.out 2>&1
sh $T/gram4-g-tools/run.sh b1 LIMIT=1 PROBES=0 ROWS_DUMP=/private/tmp/gram4-g/rows-b1 BATCHES=$S0 > $O/b1.out 2>&1
sh $T/gram4-g-tools/run.sh g1 LIMIT=1 PROBES=0 ROWS_DUMP=/private/tmp/gram4-g/rows-g1 BATCHES=$S0,$T/gram3-g-tools/staged/gc-exp > $O/g1.out 2>&1
