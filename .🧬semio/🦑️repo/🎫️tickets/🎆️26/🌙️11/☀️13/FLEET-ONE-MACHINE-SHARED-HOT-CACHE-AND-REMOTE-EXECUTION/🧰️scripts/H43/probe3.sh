#!/bin/bash
# H43: timeline of the open files of the slice worker of one narrow read (1 s lsof polling)
S=$1; LIMIT=${2:-75}; mkdir -p "$S"; cd /Users/ueli/Documents/semio || exit 1
START=$(date +%s)
perl -e 'alarm shift; exec @ARGV' "$LIMIT" nice -n 10 repo inputs repo-graph-maintenance:test >"$S/run.out" 2>"$S/run.err" &
CP=$!
W=""
while kill -0 $CP 2>/dev/null; do
  N=$(( $(date +%s) - START ))
  if [ -z "$W" ] && [ $N -ge 8 ]; then
    W=$(ps -axo pid,etime,rss,command | awk -v t=$N '$2 ~ /^00:[0-9][0-9]$/ && $3>200000 && /cache\/b\/t\/m\/bun/ {split($2,a,":"); e=a[1]*60+a[2]; d=t-5-e; if(d<0)d=-d; print d, $1}' | sort -n | head -1 | awk '{print $2}')
    echo "$N worker $W" >> "$S/timeline.txt"
  fi
  if [ -n "$W" ]; then echo "$N $(lsof -p $W -Fn 2>/dev/null | /usr/bin/grep -a '^n/Users' | sed 's|^n/Users/ueli/Documents/semio/||' | /usr/bin/grep -av 'node_modules\|\.ts$\|\.mjs$\|\.json$' | awk '{printf "%s|", substr($0,1,70)}')" >> "$S/timeline.txt"; fi
  sleep 1
done
