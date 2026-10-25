#!/bin/sh
# usage: tree-sampler.sh <rootpid> <log>   every 10 s: every descendant of <rootpid> with ppid, resident MB (ps) and footprint/compressed (top)
ROOT="$1"; LOG="$2"
kids() { for c in $(pgrep -P "$1" 2>/dev/null); do echo "$c"; kids "$c"; done; }
while kill -0 "$ROOT" 2>/dev/null; do
  for p in $(kids "$ROOT"); do
    t=$(top -l 1 -pid "$p" -stats pid,mem,cmprs 2>/dev/null | /usr/bin/grep "^$p ")
    printf "%s pid=%s ppid=%s rssMB=%s top=[%s] cmd=%s\n" "$(date +%H:%M:%S)" "$p" "$(ps -o ppid= -p $p | tr -d ' ')" "$(( $(ps -o rss= -p $p | tr -d ' ' ) / 1024 ))" "$t" "$(ps -o command= -p $p | cut -c1-60 | tr -s ' ' )" >> "$LOG"
  done
  sleep 10
done
