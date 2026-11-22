#!/bin/sh
# 🔎 person-cooking: build the candidate and run the connectivity rule of every scale (fast inner loop).
cd "$(dirname "$0")"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
F="🗑️generated/rework-person-cooking/person-cooking/🔣️.json"
python3 rework-person-cooking-girdle.py > /dev/null || exit 1
for N in 50 100 200 500; do "$BUN" r3-validate.ts --file "$F" --scale $N --rules "${1:-connected}" --no-print 2>&1 | cut -c1-${2:-900}; done
