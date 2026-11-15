#!/bin/zsh
# usage: runall.sh <overlay name> <tag> <worlds json> ; 8 shards over all recipes, 4 at a time, outputs scratch/R29/<tag>-<i>.json
HERE=${0:A:h}
S=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R29
cd /Users/Shared
export OVL=$1 R29_RECIPES=${R29_RECIPES:-$S/recipes-all.json} R29_STRIDE=8 R29_WORLDS=$3 R29_KEYS=${R29_KEYS:-0} R29_SETS=${R29_SETS:-1}
for wave in 0 4; do
  for o in 0 1 2 3; do i=$((wave+o)); R29_OFFSET=$i R29_OUT=$S/$2-$i.json zsh $HERE/run.sh all.ts > $S/$2-$i.log 2>&1 & done
  wait
done
tail -qn1 $S/$2-?.log
