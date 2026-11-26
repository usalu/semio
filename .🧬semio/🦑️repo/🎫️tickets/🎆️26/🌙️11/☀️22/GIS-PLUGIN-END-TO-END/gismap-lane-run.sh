#!/usr/bin/env bash
# usage: gismap-lane-run.sh <name> <repo args after --configuration=prototype -->...
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GIS-PLUGIN-END-TO-END"
L="$T/🗑️generated/compile.lock"
name="$1"; shift
until mkdir "$L" 2>/dev/null; do
  if [ $(( $(date +%s) - $(stat -f %m "$L" 2>/dev/null || date +%s) )) -gt 7200 ]; then rmdir "$L" 2>/dev/null; fi
  sleep 20
done
echo gismap-lane > "$L/owner"
trap 'rm -rf "$L"' EXIT
cd /Users/ueli/Documents/semio
"$@" > "$T/🗑️generated/gismap-$name.log" 2>&1
echo "exit $?" >> "$T/🗑️generated/gismap-$name.log"
