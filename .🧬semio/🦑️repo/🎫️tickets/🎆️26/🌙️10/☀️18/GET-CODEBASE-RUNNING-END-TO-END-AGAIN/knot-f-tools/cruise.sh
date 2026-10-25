#!/bin/sh
# usage: cruise.sh <name> <entries-file> — direct no-circular measurement (dependency-cruiser, repo policy options)
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
cd /Users/ueli/Documents/semio || exit 1
s=$(date +%s)
nice -n 10 bun "$C/knot-f-tools/cruise-circular.ts" "$2" "$C/🗑️generated/knot-f/$1" > "$C/🗑️generated/knot-f/$1.out" 2>&1
echo "cruise $1 rc=$? $(( $(date +%s)-s ))s" >> "$C/🗑️generated/knot-f/verdicts.txt"
