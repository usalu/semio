#!/bin/zsh
# usage: runworlds.sh <overlay name> <tag> ; the counterfactual worlds on the whole owner set, four processes in parallel (base, all30, unreferenced, individual), outputs scratch/R29/<tag>-<name>.json
HERE=${0:A:h}
S=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R29
cd /Users/Shared
export OVL=$1 R29_RECIPES=$S/recipes-all.json R29_STRIDE=1 R29_OFFSET=0
for pair in ${=PAIRS:-base1:1 all30:1 unref:1 ind0:0 ind1:0 ind2:0 ind3:0 ind4:0}; do
  n=${pair%%:*}; k=${pair##*:}
  R29_WORLDS=$S/w-$n.json R29_KEYS=$k R29_SETS=0 R29_OUT=$S/$2-$n.json zsh $HERE/run.sh all.ts > $S/$2-$n.log 2>&1 &
done
wait
for pair in ${=PAIRS:-base1:1 all30:1 unref:1 ind0:0 ind1:0 ind2:0 ind3:0 ind4:0}; do tail -qn2 $S/$2-${pair%%:*}.log; done
