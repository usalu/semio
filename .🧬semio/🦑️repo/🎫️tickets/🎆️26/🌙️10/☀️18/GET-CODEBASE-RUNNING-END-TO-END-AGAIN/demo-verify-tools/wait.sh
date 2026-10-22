#!/bin/sh
# usage: wait.sh <name> [iterations]  (10s each)
C="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/demo-verify"
for i in $(seq 1 ${2:-55}); do [ -f "$C/$1.rc" ] && break; sleep 10; done
[ -f "$C/$1.rc" ] && echo "rc=$(cat $C/$1.rc) $(cat $C/$1.start)-$(cat $C/$1.end)" || echo "still running"
tail -n 15 "$C/$1.log"
