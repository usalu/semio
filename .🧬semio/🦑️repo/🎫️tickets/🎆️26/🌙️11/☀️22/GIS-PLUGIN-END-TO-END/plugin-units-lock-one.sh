#!/usr/bin/env bash
cd /Users/ueli/Documents/semio
G=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GIS-PLUGIN-END-TO-END/🗑️generated"
export SEMIO_LANGUAGE=en
lock="$1"; d=$(dirname "$lock"); n=$(echo "$lock" | cksum | cut -d' ' -f1)
until [ "$(memory_pressure | awk '/free percentage/{gsub("%","",$5);print $5}')" -ge 20 ]; do sleep 15; done
s=$(date +%s)
timeout 900 repo cargo --store=shared --configuration=prototype -- update --offline --workspace --manifest-path "$d/Cargo.toml" > "$G/lock-refresh-$n.log" 2>&1
echo "$? $(( $(date +%s) - s ))s $lock" >> "$G/lock-refresh-status.txt"
