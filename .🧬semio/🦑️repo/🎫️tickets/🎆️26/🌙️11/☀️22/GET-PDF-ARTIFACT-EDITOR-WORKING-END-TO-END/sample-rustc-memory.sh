#!/bin/sh
# usage: sample-rustc-memory.sh <log> <stop-file> [lane]  (every 20 s: `top -l 1 -o mem -n 6 -stats pid,command,mem` plus crate, target and physical footprint of every running rustc, until <stop-file> exists;
# every 5 s a guard stops ONLY this build's own rustc (its command names `lane-<lane>`) when system free memory drops below 12 %, so the build can never drive macOS into pausing other apps)
log=$1; stop=$2; lane=${3:-w}
: > "$log"
tick=0
while [ ! -e "$stop" ]; do
  free=$(memory_pressure 2>/dev/null | awk -F': ' '/free percentage/ {gsub("%","",$2); print $2}')
  if [ -n "$free" ] && [ "$free" -lt 12 ]; then
    for pid in $(pgrep -x rustc 2>/dev/null); do
      if ps -o command= -p "$pid" 2>/dev/null | grep -q "native-target/lane-$lane/"; then
        echo "GUARD $(date +%T) free=$free% stopping own rustc $pid $(ps -o command= -p "$pid" | sed -n 's/.*--crate-name \([^ ]*\).*/\1/p') $(top -l 1 -pid "$pid" -stats pid,mem | awk -v p="$pid" '$1==p {print $2}')" >> "$log"
        kill -9 "$pid"
      fi
    done
  fi
  if [ $((tick % 4)) -eq 0 ]; then
    echo "== $(date +%T) free=$free%" >> "$log"
    top -l 1 -o mem -n 6 -stats pid,command,mem 2>/dev/null | tail -7 >> "$log"
    for pid in $(pgrep -x rustc 2>/dev/null); do
      command=$(ps -o command= -p "$pid" 2>/dev/null)
      crate=$(echo "$command" | sed -n 's/.*--crate-name \([^ ]*\).*/\1/p')
      target=$(echo "$command" | sed -n 's/.*--target \([^ ]*\).*/\1/p')
      mem=$(top -l 1 -pid "$pid" -stats pid,mem 2>/dev/null | awk -v p="$pid" '$1==p {print $2}')
      echo "rustc $pid $crate ${target:-host} $mem" >> "$log"
    done
  fi
  tick=$((tick + 1))
  sleep 5
done
