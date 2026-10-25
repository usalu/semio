#!/bin/bash
# Ticket input script: runs a command, and runs it again after two minutes while a peer's half-finished dependency change (a lock file out of date) refuses it.
#   retry-lock.sh <log> <command...>
log="$1"; shift
for attempt in 1 2 3 4 5 6 7 8; do
  echo "[retry-lock] attempt $attempt $(date +%H:%M:%S)" >> "$log"
  "$@" >> "$log" 2>&1
  code=$?
  [ "$code" = 0 ] && exit 0
  tail -n 60 "$log" | /usr/bin/grep -aq "lock file of dependency scope\|refused the frozen installation" || exit "$code"
  sleep 120
done
exit 75
