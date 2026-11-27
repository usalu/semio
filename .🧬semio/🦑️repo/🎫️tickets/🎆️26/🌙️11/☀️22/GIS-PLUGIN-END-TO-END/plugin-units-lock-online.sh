#!/usr/bin/env bash
cd /Users/ueli/Documents/semio
G=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GIS-PLUGIN-END-TO-END/🗑️generated"
export SEMIO_LANGUAGE=en
: > "$G/lock-online-status.txt"
while IFS= read -r lock; do
  d=$(dirname "$lock"); s=$(date +%s)
  timeout 300 repo cargo --store=shared --configuration=prototype -- update --workspace --manifest-path "$d/Cargo.toml" > "$G/lock-online-last.log" 2>&1
  echo "$? $(( $(date +%s)-s ))s $lock" >> "$G/lock-online-status.txt"
done < "$G/stale-locks-now.txt"
echo done >> "$G/lock-online-status.txt"
