#!/bin/sh
# usage: wait.sh <rawlog> <pid> [loops] -> waits up to loops*10 s for rc= or pid exit; prints progress tail
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
cd /Users/ueli/Documents/semio || exit 1
i=0; while [ $i -lt ${3:-55} ]; do kill -0 $2 2>/dev/null || break; /usr/bin/grep -q '^rc=' "$1" && break; sleep 10; i=$((i+1)); done
tail -c 1500 "$1" | tail -6 | cut -c1-260
df -h .cache/b/l/m | tail -1
