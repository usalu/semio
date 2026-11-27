#!/usr/bin/env bash
cd /Users/ueli/Documents/semio
G=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GIS-PLUGIN-END-TO-END/🗑️generated"
export SEMIO_LANGUAGE=en
: > "$G/leaseorder-status.txt"
n=0
for lock in $(head -4 "$G/stale-locks.txt" | tr ' ' '\001'); do
  lock=$(echo "$lock" | tr '\001' ' '); n=$((n+1)); d=$(dirname "$lock")
  ( s=$(date +%s); timeout 600 repo cargo --store=shared --configuration=prototype -- update --offline --workspace --manifest-path "$d/Cargo.toml" > "$G/leaseorder-$n.log" 2>&1; echo "$? $(( $(date +%s) - s ))s $lock" >> "$G/leaseorder-status.txt" ) &
done
wait
