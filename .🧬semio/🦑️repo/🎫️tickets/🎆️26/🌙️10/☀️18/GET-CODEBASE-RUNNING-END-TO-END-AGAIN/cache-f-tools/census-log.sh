#!/bin/sh
# usage: census-log.sh <fragment-of-file-path> -> failing assertions of the k7 census log of the first matching file
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
G="$C/🗑️generated/cache-d/k7"
l=$(/usr/bin/grep -F "$1" "$G/summary.txt" | head -1); echo "### $l" | cut -c1-200
lg=$(echo "$l" | sed 's/.*log=//')
/usr/bin/grep -v '^(pass)' "$G/$lg" | /usr/bin/grep -E -B2 -A${A:-12} '^error:' | cut -c1-${W:-260} | head -${N:-70}
