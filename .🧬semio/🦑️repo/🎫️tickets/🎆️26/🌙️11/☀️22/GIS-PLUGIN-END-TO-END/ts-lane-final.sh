#!/bin/bash
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GIS-PLUGIN-END-TO-END"
G="$T/🗑️generated"
cd /Users/ueli/Documents/semio
for p in @semio-tech/s-plugin-composition:test @semio-tech/gis-gisterrain:test @semio-tech/gis-gismap-js:test @semio-tech/gis-js:test; do
  until mkdir "$G/compile.lock4" 2>/dev/null; do if [ $(( $(date +%s) - $(stat -f %m "$G/compile.lock4") )) -gt 7200 ]; then rmdir "$G/compile.lock4"; fi; sleep 20; done; echo ts-lane > "$G/compile.lock4/owner"
  n=$(echo "${p%%:*}" | tr '/' '_')
  env -u SEMIO_REMOTE_CACHE_TOKEN -u SEMIO_REMOTE_CACHE_ENDPOINT SEMIO_OFFLOAD=distribute timeout 7200 repo run "$p" --lang=en > "$G/ts-last-$n.log" 2>&1
  echo "$p rc=$?" >> "$G/ts-last-results.txt"
  rm -rf "$G/compile.lock4"
done
echo done >> "$G/ts-last-results.txt"
