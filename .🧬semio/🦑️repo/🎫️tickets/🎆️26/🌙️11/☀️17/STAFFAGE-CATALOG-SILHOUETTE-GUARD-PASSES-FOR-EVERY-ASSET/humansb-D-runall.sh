#!/bin/zsh
# 🏃 Runs the greedy repair for assets and scales in parallel (at most 4 at a time). usage: humansb-D-runall.sh "<assets>" "<scales>" [maxIter]
source /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-CATALOG-SILHOUETTE-GUARD-PASSES-FOR-EVERY-ASSET/humansb-D-lib.sh
cd $TS
for a in ${=1}; do for s in ${=2}; do
  while [ $(pgrep -f "humansb-D-auto.ts" | wc -l) -ge 4 ]; do sleep 3; done
  $B $T/humansb-D-auto.ts $a $s $G/A-$a-$s.json ${3:-120} > $G/A-$a-$s.log 2>&1 &
  sleep 1
done; done
wait
