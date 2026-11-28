#!/bin/bash
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GIS-PLUGIN-END-TO-END"
G="$T/🗑️generated"
cd /Users/ueli/Documents/semio
until grep -q "^done" "$G/ts-rerun-results.txt"; do sleep 30; done
for p in @semio-tech/gis-gismap-js:test; do
  until mkdir "$G/compile.lock4" 2>/dev/null; do if [ $(( $(date +%s) - $(stat -f %m "$G/compile.lock4") )) -gt 7200 ]; then rmdir "$G/compile.lock4"; fi; sleep 20; done; echo ts-lane > "$G/compile.lock4/owner"
  n=$(echo "${p%%:*}" | tr '/' '_')
  env -u SEMIO_REMOTE_CACHE_TOKEN -u SEMIO_REMOTE_CACHE_ENDPOINT SEMIO_OFFLOAD=distribute timeout 7200 repo run "$p" --lang=en > "$G/ts-rerun2-$n.log" 2>&1
  echo "$p rerun rc=$?" >> "$G/ts-rerun2-results.txt"
  rm -rf "$G/compile.lock4"
done
echo done >> "$G/ts-rerun2-results.txt"
