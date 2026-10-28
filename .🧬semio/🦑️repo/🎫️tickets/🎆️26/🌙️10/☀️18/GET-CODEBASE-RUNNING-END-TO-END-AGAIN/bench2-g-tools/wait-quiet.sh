#!/bin/sh
# wait-quiet.sh [seconds]: polls coordination.md for "QUIET WINDOW START" (default 520 s).
C="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
left=${1:-520}
while [ "$left" -gt 0 ]; do
  row=$(/usr/bin/grep "QUIET WINDOW START" "$C/coordination.md" | tail -1)
  [ -n "$row" ] && { echo "$row" | cut -c1-400; exit 0; }
  sleep 30; left=$((left-30))
done
echo "no QUIET WINDOW START yet $(date +%H:%M)"; exit 1
