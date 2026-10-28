#!/bin/sh
# m4: course vitest suite, graph-free direct, peak tree RSS at several worker counts (rows m4.jsonl)
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
O="$C/🗑️generated/testmem-g"
R="/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/📚️course/📦️packages/🟦️typescript"
for w in ${WORKERS:-1 2 4 8 default}; do
 TREE_RSS_INTERVAL=0.2 python3 "$C/testmem-g-tools/vt-run.py" "m4-course-w$w" "$O/m4-w$w.json" "$R" "../../🧪️tests/🎚️config/🟦️.ts" "$w" >> "$O/m4.jsonl" 2>&1
done
