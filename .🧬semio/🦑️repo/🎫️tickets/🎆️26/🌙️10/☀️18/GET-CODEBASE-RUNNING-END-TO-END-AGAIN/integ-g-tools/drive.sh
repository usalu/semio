#!/bin/sh
# usage: drive.sh <first-cycle-number> [period-seconds=4500] ; endless: per cycle dash, print, course one at a time, each waits (poll 120 s) until < 4 waiters
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
O="$C/🗑️generated/integ-g"; mkdir -p "$O"
n="${1:-1}"; period="${2:-4500}"
while [ ! -f "$O/STOP" ]; do
  t0=$(date +%s)
  for prod in dash print course; do
    [ -f "$O/STOP" ] && break
    while :; do
      sh "$C/integ-g-tools/cycle.sh" "c$n" "$prod"; rc=$?
      [ $rc -ne 3 ] && break
      [ -f "$O/STOP" ] && break
      sleep 120
    done
  done
  n=$((n+1))
  while [ $(( $(date +%s) - t0 )) -lt "$period" ] && [ ! -f "$O/STOP" ]; do sleep 30; done
done
