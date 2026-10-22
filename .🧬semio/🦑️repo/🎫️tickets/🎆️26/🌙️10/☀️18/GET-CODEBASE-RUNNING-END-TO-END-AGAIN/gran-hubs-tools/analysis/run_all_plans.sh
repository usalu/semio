#!/bin/sh
# 📝 Plans and assembles the three facade repoints one after the other (cwd independent; one process at a time).
cd "$(dirname "$0")" || exit 1
for h in "$@"; do
  nice -n 10 python3 plan_hub.py "$h" 2>&1 | tail -2
  nice -n 10 python3 assemble2.py "$h" 2>&1 | tail -2
done
