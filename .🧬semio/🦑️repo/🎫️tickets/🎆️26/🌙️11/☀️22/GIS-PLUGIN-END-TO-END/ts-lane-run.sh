#!/bin/bash
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GIS-PLUGIN-END-TO-END"
G="$T/🗑️generated"
cd /Users/ueli/Documents/semio
until grep -q '^done' "$G/gen-results.txt"; do sleep 20; done
for p in @semio-tech/gis-gisterrain:test @semio-tech/gis-gismap-js:test @semio-tech/s-plugin-composition:test @semio-tech/gis-js:test; do
  until mkdir "$G/compile.lock" 2>/dev/null; do if [ $(( $(date +%s) - $(stat -f %m "$G/compile.lock") )) -gt 7200 ]; then rmdir "$G/compile.lock"; fi; sleep 20; done; echo ts-lane > "$G/compile.lock/owner"
  n=$(echo "${p%%:*}" | tr '/' '_')
  env -u SEMIO_REMOTE_CACHE_TOKEN -u SEMIO_REMOTE_CACHE_ENDPOINT SEMIO_OFFLOAD=distribute timeout 3000 repo run "$p" --lang=en > "$G/ts-final-$n.log" 2>&1
  echo "$p rc=$?" >> "$G/ts-final-results.txt"
  rm -rf "$G/compile.lock"
done
echo done >> "$G/ts-final-results.txt"
