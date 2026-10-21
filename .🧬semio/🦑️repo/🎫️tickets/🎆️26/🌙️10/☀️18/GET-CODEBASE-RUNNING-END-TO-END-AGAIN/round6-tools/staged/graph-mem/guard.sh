#!/bin/sh
# waits until the one minute load average is below 30 (the coordinator's guard before any process over 200 MB)
while :; do
  load=$(sysctl -n vm.loadavg | awk '{print int($2)}')
  [ "$load" -lt 30 ] && exit 0
  echo "guard: load $load, waiting 60 s" >&2; sleep 60
done
