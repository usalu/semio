#!/usr/bin/env bash
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GIS-PLUGIN-END-TO-END"
L="$T/🗑️generated/compile.lock"
name="$1"; shift
cd /Users/ueli/Documents/semio
G=/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌍️gis
for attempt in 1 2 3 4 5 6; do
  until mkdir "$L" 2>/dev/null; do
    if [ $(( $(date +%s) - $(stat -f %m "$L" 2>/dev/null || date +%s) )) -gt 7200 ]; then rmdir "$L" 2>/dev/null; fi
    sleep 20
  done
  echo gismap2-lane > "$L/owner"
  until [ "$(memory_pressure | awk '/free percentage/{gsub("%","",$5);print $5}')" -ge 25 ]; do sleep 20; done
  "$@" > "$T/🗑️generated/gismap2-$name.log" 2>&1
  rc=$?
  rm -rf "$L"
  sleep 120
  grep -q "host warden cancelled\|Wächter\] der Build kann nicht enden\|critically short of memory" "$T/🗑️generated/gismap2-$name.log" || break
done
echo "exit $rc" >> "$T/🗑️generated/gismap2-$name.log"
