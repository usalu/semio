#!/bin/bash
# usage: w.sh <name> [max-iterations of 10s]
C="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
for i in $(seq 1 ${2:-55}); do [ -f "$C/🗑️generated/1029/rtb-$1.rc" ] && break; sleep 10; done
ls "$C/🗑️generated/1029/rtb-$1.rc" 2>/dev/null; cat "$C/🗑️generated/1029/rtb-$1.rc" 2>/dev/null; tail -3 "$C/🗑️generated/1029/rtb-$1.txt" | cut -c1-300
