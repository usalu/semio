#!/usr/bin/env bash
cd /Users/ueli/Documents/semio
G=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GIS-PLUGIN-END-TO-END/🗑️generated"
export SEMIO_LANGUAGE=en
: > "$G/ts-targets-locked-exit.txt"
for t in @semio-tech/gis-js:test @semio-tech/gis-gismap-js:test @semio-tech/gis-gisterrain:test @semio-tech/s-plugin-composition:test; do
  until mkdir "$G/compile.lock" 2>/dev/null; do sleep 20; done
  echo ts-lane > "$G/compile.lock/owner"
  until [ "$(memory_pressure | awk '/free percentage/{gsub("%","",$5);print $5}')" -ge 20 ]; do sleep 15; done
  n=$(echo "$t" | tr '/:@' '___')
  s=$(date +%s)
  timeout 3000 repo run "$t" > "$G/ts-locked-$n.log" 2>&1
  c=$?
  echo "$t $c $(( $(date +%s) - s ))s" >> "$G/ts-targets-locked-exit.txt"
  rm -f "$G/compile.lock/owner"; rmdir "$G/compile.lock"
  sleep 120
done
echo done >> "$G/ts-targets-locked-exit.txt"
