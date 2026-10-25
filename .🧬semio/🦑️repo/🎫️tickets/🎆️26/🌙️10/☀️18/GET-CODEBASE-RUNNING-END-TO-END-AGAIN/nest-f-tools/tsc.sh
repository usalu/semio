#!/bin/sh
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
start=$(date +%s)
nice -n 10 node_modules/.bin/tsc -p "$C/nest-f-tools/tsconfig.nest.json" > "$C/🗑️generated/nest-f/tsc.out" 2>&1
echo "rc=$? seconds=$(( $(date +%s) - start ))" > "$C/🗑️generated/nest-f/tsc.rc"
