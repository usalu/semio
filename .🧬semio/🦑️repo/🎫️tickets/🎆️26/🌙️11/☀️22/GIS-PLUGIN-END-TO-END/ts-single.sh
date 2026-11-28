#!/bin/bash
# usage: ts-single.sh project... ; runs each test target alone, recording exit codes
G="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GIS-PLUGIN-END-TO-END/🗑️generated"
cd /Users/ueli/Documents/semio
for p in "$@"; do
  n=${p##*/}
  for a in 1 2 3; do
    env -u SEMIO_LEASE_OWNER -u SEMIO_REMOTE_CACHE_TOKEN -u SEMIO_REMOTE_CACHE_ENDPOINT SEMIO_OFFLOAD=distribute repo run "$p:test" --lang=en > "$G/single-$n-$a.log" 2>&1
    rc=$?; echo "$(date +%H:%M) $p attempt $a rc=$rc" >> "$G/single-results.txt"
    grep -qE "Successfully ran|Failed tasks|Test Files|Tests +[0-9]" "$G/single-$n-$a.log" && break
  done
done
