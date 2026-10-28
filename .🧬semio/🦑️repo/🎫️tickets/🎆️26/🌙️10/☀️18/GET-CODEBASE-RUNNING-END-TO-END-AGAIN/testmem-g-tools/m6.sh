#!/bin/sh
# m6: course vitest, graph-free direct; scenarios default | w4 (ts-g constant) | host | grant2048 | grant1300 (rows m6.jsonl)
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
O="$C/🗑️generated/testmem-g"
R="/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/📚️course/📦️packages/🟦️typescript"
for w in ${SCENARIOS:-default w4 host grant2048 grant1300}; do
 TREE_RSS_INTERVAL=0.2 python3 "$C/testmem-g-tools/vt-run.py" "m6-course-$w" "$O/m6-$w.json" "$R" "../../🧪️tests/🎚️config/🟦️.ts" "$w" >> "$O/m6.jsonl" 2>&1
done
