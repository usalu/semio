#!/bin/sh
# Waits for the host gate, then runs one cargo job at parallelism 1. Usage: dwg-test-when-quiet.sh <log> <cargo args...>
log=$1; shift
root=/Users/ueli/Documents/semio
while :; do
  free=$(memory_pressure | tail -1 | sed 's/[^0-9]*\([0-9]*\)%.*/\1/')
  swap=$(sysctl -n vm.swapusage | sed 's/.*used = \([0-9.]*\)M.*/\1/' | cut -d. -f1)
  others=$(pgrep -f 'rustc ' | wc -l | tr -d ' ')
  if [ "$swap" -lt 3500 ] && [ "$free" -ge 45 ] && [ "$others" -eq 0 ]; then break; fi
  echo "waiting free=$free swap=${swap}M rustc=$others" > "$log.wait"
  sleep 20
done
cd "$root" && repo cargo --store=shared --configuration=prototype --parallelism=1 -- "$@" > "$log" 2>&1
echo "exit=$?" >> "$log"
