#!/bin/sh
# Ticket input script: reruns the play activation until it exits 0 (Bazel's disk cache keeps every green action), at most $2 attempts.
#   loop.sh <log-prefix> <max-attempts> [extra repo args]
prefix=$1; max=$2; shift 2
n=0
while [ "$n" -lt "$max" ]; do
  n=$((n+1))
  echo "[loop] attempt $n at $(date +%H:%M:%S)" >> "$prefix.loop.log"
  SEMIO_BUILD_GRAPH_POLICY=current nice -n 10 bun repo --tool=bazel --workers=off run @semio-tech/semio-tech-play:activate-prototype "$@" > "$prefix.$n.log" 2>&1
  rc=$?
  echo "[loop] attempt $n rc=$rc at $(date +%H:%M:%S)" >> "$prefix.loop.log"
  [ "$rc" -eq 0 ] && exit 0
  sleep 30
done
exit 1
