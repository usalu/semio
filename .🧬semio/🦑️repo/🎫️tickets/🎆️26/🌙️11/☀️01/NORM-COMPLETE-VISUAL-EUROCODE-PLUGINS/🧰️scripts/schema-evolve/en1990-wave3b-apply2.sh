#!/usr/bin/env bash
cd "$(dirname "$0")"
until [ -e ../../🗑️generated/en1990b/evolve-done.txt ]; do sleep 20; done
HEAVY_MAX_WAIT=2700 ../../🗑️generated/build-runner/heavy.sh nice -n 19 python3 -I evolve.py add-leaf en1990 reliabilityAnalyses specs/en1990-reliability-member.json --apply 2>&1 | tail -3
echo done > ../../🗑️generated/en1990b/evolve2-done.txt
