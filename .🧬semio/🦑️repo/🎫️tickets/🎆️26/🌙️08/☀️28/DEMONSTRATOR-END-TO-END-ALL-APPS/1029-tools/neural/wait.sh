#!/bin/bash
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
n="$1"
for i in $(seq 1 55); do [ -f "$C/🗑️generated/1029/neural-$n.rc" ] && break; sleep 10; done
ls "$C/🗑️generated/1029/neural-$n.rc" 2>/dev/null && cat "$C/🗑️generated/1029/neural-$n.rc"
tail -3 "$C/🗑️generated/1029/neural-$n.txt" | cut -c1-300
