#!/bin/sh
# usage: retry.sh <max> <log-prefix> <cmd...>; retries only on transient concurrent-edit faults
max=$1; pre=$2; shift 2
cd /Users/ueli/Documents/semio || exit 1
n=1
while [ $n -le $max ]; do
  log="$pre.$n.log"
  "$@" > "$log" 2>&1
  rc=$?
  echo "attempt $n rc=$rc $(date +%H:%M:%S)" >> "$pre.attempts"
  [ $rc -eq 0 ] && exit 0
  if /usr/bin/grep -qE "Stale loaded|DATA directory changed|exited with code 75|Source changed|settle" "$log"; then n=$((n+1)); sleep 20; else exit $rc; fi
done
exit 1
