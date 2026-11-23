#!/bin/sh
# 🛡️ Stops own private-root build processes (those whose command line names the private root) when free space on the data volume drops below 5 GiB.
P="${1:-$HOME/.sinc/g3d}"
while :; do
  free=$(df -k /System/Volumes/Data | awk 'NR==2{print int($4/1048576)}')
  if [ "$free" -lt 5 ]; then echo "[watchdog] free ${free} GiB < 5: stopping builds under $P"; pkill -TERM -f "$P"; sleep 20; pkill -KILL -f "$P"; exit 1; fi
  sleep 15
done
