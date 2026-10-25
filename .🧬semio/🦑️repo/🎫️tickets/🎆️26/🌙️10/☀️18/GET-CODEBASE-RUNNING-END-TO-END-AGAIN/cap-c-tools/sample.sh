#!/bin/sh
# usage: sample.sh <out> <seconds> : every 5 s the summed CPU of rustc/cargo/bun-wrapper processes, load, and the compiler capacity lease rows
cd /Users/ueli/Documents/semio || exit 1
end=$(( $(date +%s) + $2 ))
while [ $(date +%s) -lt $end ]; do
  cpu=$(ps -A -o pcpu=,command= | /usr/bin/grep -E "rustc|/cargo |semio-build-scheduler|wasm-ld|cc " | /usr/bin/grep -v grep | awk '{s+=$1} END {printf "%.0f", s}')
  echo "$(date +%H:%M:%S) rustcCpu=${cpu}% $(uptime | sed 's/.*load/load/')" >> "$1"
  bun repo leases status 2>/dev/null | /usr/bin/grep -E "host-capacity|GRANTED" | sed 's/^/   /' >> "$1"
  sleep 5
done
