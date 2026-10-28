#!/bin/sh
# Warm no-op baseline of the three products on the live tree (read-only: no file is edited). One run at a time.
# Waits (up to 60 min) until the CLI itself is healthy: a half-edited registry file of a peer makes every run exit 1/2.
C="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
O="$C/🗑️generated/bench-g/live"
mkdir -p "$O"
cd /Users/ueli/Documents/semio || exit 1
n=0
until bun repo --help >/dev/null 2>&1; do n=$((n+1)); [ $n -gt 60 ] && echo "CLI unhealthy for 60 min" && exit 3; sleep 60; done
for p in dashboard course print; do
  env -u SEMIO_SCRIPT_STALL_MS python3 "$C/bench-g-tools/measure.py" noop --product $p --root /Users/ueli/Documents/semio --tool bazel --out "$O" --prime --samples 3
done
