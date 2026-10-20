#!/bin/sh
# 📝 Plans the three facade repoints one after the other (cwd independent).
cd "$(dirname "$0")" || exit 1
for h in "$@"; do nice -n 10 python3 plan_hub.py "$h" 2>&1 | tail -3; done
