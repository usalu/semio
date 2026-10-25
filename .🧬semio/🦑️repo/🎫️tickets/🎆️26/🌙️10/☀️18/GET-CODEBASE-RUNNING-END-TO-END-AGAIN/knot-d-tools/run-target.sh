#!/bin/sh
# usage: run-target.sh <name> <repo args...>
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
cd /Users/ueli/Documents/semio || exit 1
name="$1"; shift
s=$(date +%s)
SEMIO_LANGUAGE=en nice -n 10 bun repo "$@" > "$C/🗑️generated/knot-d/$name.log" 2>&1
rc=$?
echo "$name rc=$rc $(( $(date +%s)-s ))s" >> "$C/🗑️generated/knot-d/verdicts.txt"
