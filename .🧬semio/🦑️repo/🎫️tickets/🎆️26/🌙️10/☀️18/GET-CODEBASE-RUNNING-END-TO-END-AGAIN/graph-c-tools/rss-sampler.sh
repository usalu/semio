#!/bin/sh
# usage: rss-sampler.sh <log>   every 10 s: pid, ppid, footprint (top MEM) and compressed (CMPRS) of every bun process above 300 MB
LOG="$1"
while true; do
  top -l 1 -n 60 -o mem -stats pid,command,mem,cmprs 2>/dev/null | /usr/bin/grep "^[0-9]" | /usr/bin/grep "bun" | while read pid cmd mem cmprs; do
    printf "%s pid=%s ppid=%s mem=%s cmprs=%s\n" "$(date +%H:%M:%S)" "$pid" "$(ps -o ppid= -p $pid | tr -d ' ')" "$mem" "$cmprs" >> "$LOG"
  done
  sleep 10
done
