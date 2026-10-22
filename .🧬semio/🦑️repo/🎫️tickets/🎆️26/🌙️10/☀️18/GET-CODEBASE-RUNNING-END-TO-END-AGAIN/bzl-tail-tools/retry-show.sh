#!/bin/sh
# usage: retry-show.sh <log> <attempts>  -> retries `show targets` until exit 0; writes attempt summaries to <log>
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
log=$1; n=$2; i=1
while [ "$i" -le "$n" ]; do
  SEMIO_BUILD_GRAPH_POLICY=current nice -n 10 bun repo --tool=bazel show targets --json > "$C/🗑️generated/bzl-tail/retry-$i.raw" 2>&1
  rc=$?
  echo "attempt $i rc=$rc $(date +%H:%M:%S)" >> "$log"
  /usr/bin/grep -m3 '^error' "$C/🗑️generated/bzl-tail/retry-$i.raw" | cut -c1-300 >> "$log"
  [ "$rc" = 0 ] && exit 0
  i=$((i+1)); sleep 60
done
exit 1
