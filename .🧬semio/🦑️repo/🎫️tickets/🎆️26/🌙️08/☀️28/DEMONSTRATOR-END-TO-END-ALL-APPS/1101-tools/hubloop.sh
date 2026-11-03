#!/bin/bash
# usage: hubloop.sh — repeat the full hub check until it passes, then write the marker
D="$(dirname "$0")"; cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"; G="$C/🗑️generated/1101"
n=1
while :; do
  "$D/hub.sh" "hubl-$n"
  if [ "$(cat "$G/hubl-$n.rc")" = "0" ]; then echo "hubl-$n $(date '+%F %T')" > "$G/hub-green"; exit 0; fi
  /usr/bin/grep -E '^error: could not compile' "$G/hubl-$n.txt" | sed -E 's/error: could not compile `(.*)`.*due to ([0-9]+).*/\1 \2/' | sort -u > "$G/hub-frontier.txt"
  n=$((n+1)); sleep 240
done
