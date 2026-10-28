#!/bin/sh
# usage: why.sh <fragment of build-relative path> [lines-before] [lines-after] -> first failing assertion context of the census log
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"; G="$C/🗑️generated/route-g"
for d in c3 c2 c4 c5 c6; do [ -f "$G/$d/summary.txt" ] || continue; l=$(/usr/bin/grep -F "$1" "$G/$d/summary.txt" | tail -1); [ -n "$l" ] && break; done
log=$(echo "$l" | sed 's/.*log=//;s/\t.*//'); echo "### $l" | cut -c1-200
/usr/bin/grep -v '^(pass)' "$G/$d/$log" | /usr/bin/grep -v '^\[script\]' | /usr/bin/grep -E -m1 -B${2:-4} -A${3:-10} '^(error|\w*Error):' | cut -c1-${W:-420}
