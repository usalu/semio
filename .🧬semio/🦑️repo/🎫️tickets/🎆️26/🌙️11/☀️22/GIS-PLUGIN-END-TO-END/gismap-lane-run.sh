#!/usr/bin/env bash
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GIS-PLUGIN-END-TO-END"
L="$T/🗑️generated/compile.lock"
name="$1"; shift
cd /Users/ueli/Documents/semio
for attempt in 1 2 3 4 5; do
  until mkdir "$L" 2>/dev/null; do
    if [ $(( $(date +%s) - $(stat -f %m "$L" 2>/dev/null || date +%s) )) -gt 7200 ]; then rmdir "$L" 2>/dev/null; fi
    sleep 20
  done
  echo gismap-lane > "$L/owner"
  "$@" > "$T/🗑️generated/gismap-$name.log" 2>&1
  rc=$?
  rm -rf "$L"
  grep -q "host warden cancelled\|Wächter\] der Build kann nicht enden" "$T/🗑️generated/gismap-$name.log" || break
  sleep 90
done
echo "exit $rc" >> "$T/🗑️generated/gismap-$name.log"
