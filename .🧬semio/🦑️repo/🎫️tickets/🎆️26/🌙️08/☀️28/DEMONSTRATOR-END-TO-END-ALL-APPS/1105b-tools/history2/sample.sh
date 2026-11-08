#!/bin/bash
# usage: sample.sh <label>  samples the running semio_os_history test binary (stack of the hang) into 🗑️generated/1105b/<label>-sample.txt
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
for i in $(seq 1 120); do
  pid=$(pgrep -f 'build/semio-os-history/.*/out/semio_os_history-' | head -1)
  [ -n "$pid" ] && break
  sleep 5
done
[ -z "$pid" ] && exit 1
sleep "${2:-60}"
/usr/bin/sample "$pid" 5 -file "$C/🗑️generated/1105b/$1-sample.txt" > /dev/null 2>&1
echo "sampled $pid" > "$C/🗑️generated/1105b/$1-sample.rc"
