#!/bin/zsh
# usage: rr.sh <tag> <keys> [config] : oracle + widen loop (2 rounds) and summary of the remaining non-noise errors
TAG=$1; KEYS=$2; CFG=${3:-lib}
HERE=${0:A:h}
GEN=$HERE/../../🗑️generated
for i in 1 2; do
  $HERE/batch.sh oracle $TAG $KEYS --configs $CFG --top 400 2>&1 | cut -c1-230 > $GEN/pi1-$TAG-$CFG-r$i.log
  n=$($HERE/batch.sh widen $TAG $CFG | wc -l)
  echo "round $i: $(/usr/bin/grep -m1 '^==' $GEN/pi1-$TAG-$CFG-r$i.log) widened $n"
  [ "$n" = "0" ] && break
done
