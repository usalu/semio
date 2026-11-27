#!/usr/bin/env bash
cd /Users/ueli/Documents/semio
G=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GIS-PLUGIN-END-TO-END/🗑️generated"
export SEMIO_LANGUAGE=en
: > "$G/lock-refresh-status.txt"
while IFS= read -r lock; do
  until [ "$(memory_pressure | awk '/free percentage/{gsub("%","",$5);print $5}')" -ge 20 ]; do sleep 15; done
  d=$(dirname "$lock")
  timeout 900 repo cargo --store=shared --configuration=prototype -- update --offline --workspace --manifest-path "$d/Cargo.toml" > "$G/lock-refresh-last.log" 2>&1
  echo "$? $lock" >> "$G/lock-refresh-status.txt"
done < "$G/stale-locks.txt"
echo done >> "$G/lock-refresh-status.txt"
