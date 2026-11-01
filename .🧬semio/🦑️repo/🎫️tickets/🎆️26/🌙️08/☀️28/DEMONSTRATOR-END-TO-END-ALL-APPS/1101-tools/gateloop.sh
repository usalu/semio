#!/bin/bash
# usage: gateloop.sh — repeat the gate check until it passes, then write the marker
D="$(dirname "$0")"; cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"; G="$C/🗑️generated/1101"
n=1
while :; do
  "$D/gate.sh" "gatel-$n"
  if [ "$(cat "$G/gatel-$n.rc")" = "0" ]; then echo "gatel-$n $(date '+%F %T')" > "$G/framework-green"; exit 0; fi
  n=$((n+1)); sleep 300
done
