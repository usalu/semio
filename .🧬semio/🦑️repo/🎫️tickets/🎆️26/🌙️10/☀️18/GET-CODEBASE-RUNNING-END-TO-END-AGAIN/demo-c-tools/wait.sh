#!/bin/sh
# usage: wait.sh <rawlog> <pid> [loops] ; waits up to loops*10 s, prints progress summary
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
cd /Users/ueli/Documents/semio || exit 1
i=0; while [ $i -lt ${3:-54} ]; do kill -0 $2 2>/dev/null || break; /usr/bin/grep -q '^rc=' "$1" && break; sleep 10; i=$((i+1)); done
/usr/bin/grep -aoE '\[[0-9]+ / [0-9]+\]|waiting for one|FAILED[^"]{0,160}' "$1" | tail -4
tail -c 700 "$1" | tail -3 | cut -c1-260
uptime
