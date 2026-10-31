#!/bin/bash
cd /Users/ueli/Documents/semio
C0=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS/1030-tools/stdiofix"
O=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS/🗑️generated/1030"
for n in 1 2 3 4 5 6 7 8 9 10 11 12; do
  rm -f "$O/stdiofix-t6.txt"; bash "/Users/ueli/Documents/semio/$C0/run4.sh"
  if /usr/bin/grep -q "^test result" "$O/stdiofix-t6.txt"; then echo done > "$O/stdiofix-loop.rc"; exit 0; fi
  sleep 240
done
echo gaveup > "$O/stdiofix-loop.rc"
