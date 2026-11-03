#!/usr/bin/env bash
cd "$(dirname "$0")"
H=../../🗑️generated/build-runner/heavy.sh
for p in "testSeries test-pairs" "testSeries test-theory-variation" "testSeries test-theory-mean" "reliabilityAnalyses reliability-second-action-distribution" "reliabilityAnalyses reliability-second-action-mean" "reliabilityAnalyses reliability-second-action-sd"; do
  set -- $p
  HEAVY_MAX_WAIT=2700 $H nice -n 19 python3 -I evolve.py add-leaf en1990 $1 specs/en1990-$2.json --apply 2>&1 | tail -3
done
mkdir -p ../../🗑️generated/en1990b; echo done > ../../🗑️generated/en1990b/evolve-done.txt
