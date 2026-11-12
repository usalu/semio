#!/bin/sh
# [DEBUG] samples the RSS of the graph reading child against the last phase line of the command log; usage: repo-graph-sample.sh LOG OUT SECONDS
LOG="$1"; OUT="$2"; END=$(( $(date +%s) + $3 ))
: > "$OUT"
while [ "$(date +%s)" -lt "$END" ]; do
  PID=$(ps -axo pid,command | grep "bun --eval" | grep -v grep | grep "service host run" -v | awk '{print $1}' | tail -1)
  RSS=$(ps -p "$PID" -o rss= 2>/dev/null | tr -d ' ')
  PH=$(grep -v '^\[script\] {' "$LOG" | grep -v "still running\|remains active\|still working\|waiting for memory" | tail -1 | cut -c1-110)
  echo "$(date +%T) pid=$PID rssMB=$(( ${RSS:-0} / 1024 )) | $PH" >> "$OUT"
  sleep 3
done
