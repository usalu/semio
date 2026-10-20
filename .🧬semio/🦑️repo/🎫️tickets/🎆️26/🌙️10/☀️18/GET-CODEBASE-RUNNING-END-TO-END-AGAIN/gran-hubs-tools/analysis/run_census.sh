#!/bin/sh
# 👥 Runs the five hub censuses and the classification, one process at a time (cwd = this folder).
cd "$(dirname "$0")" || exit 1
for h in os-kernel umbrella plugin job kernel os-store-sync; do nice -n 10 python3 census.py "$h" 2>&1 | tail -1; done
python3 classify.py
