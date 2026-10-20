#!/bin/sh
# usage: guarded.sh OUTFILE cmd... ; waits until load < 30, then runs the command niced.
out="$1"; shift
while [ "$(sysctl -n vm.loadavg | awk '{print int($2)}')" -ge 30 ]; do sleep 20; done
nice -n 10 "$@" > "$out" 2>&1
echo "EXIT $?" >> "$out"
