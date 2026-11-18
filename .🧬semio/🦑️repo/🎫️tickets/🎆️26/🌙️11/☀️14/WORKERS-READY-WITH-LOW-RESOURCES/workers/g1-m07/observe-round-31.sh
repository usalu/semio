#!/bin/bash
# 🔭 Observes the ready child of g1-m07 for round 31: every 10 minutes it appends ready.json, the tail of the running stage's log, memory pressure, the five largest processes and the access receipts to round-31.md; it starts nothing and ends when ready.json says ready or after <hours> (default 8). Usage: observe-round-31.sh [hours]
set -u
M="$(cd "$(dirname "$0")" && pwd)"; O=/Volumes/semio-store/v1/origins/g1-m07; out="$M/round-31.md"; n=0; max=$(( ${1:-8} * 6 ))
A="$HOME/Library/Application Support/semio/access"
while [ $n -lt $max ]; do
  stage="$(grep -o '"stage":"[a-z-]*"' "$O/ready.json" | head -1 | cut -d'"' -f4)"
  {
    echo; echo "## $(date -u +%Y-%m-%dT%H:%M:%SZ)"
    echo '`ready.json`:'; echo '```'; cut -c1-1500 "$O/ready.json"; echo; echo '```'
    echo "Last 30 lines of \`ready-log-$stage.txt\`:"; echo '```'; sed $'s/\x1b\\[[0-9;]*m//g' "$O/ready-log-$stage.txt" 2>/dev/null | tr '\r' '\n' | tail -30 | cut -c1-400; echo '```'
    echo "Memory: $(memory_pressure | tail -1)"
    echo "Five largest processes (resident KiB, pid, command):"; echo '```'; ps -axo rss,pid,command | sort -nr | head -5 | cut -c1-300; echo '```'
    echo "Permission dialog: not measured on screen (a session cannot see it); access receipts with a waiting state: $(grep -l '"state":"waiting"' "$A"/*/*.json 2>/dev/null | wc -l | tr -d ' ')"
  } >> "$out"
  grep -q '"ready":true' "$O/ready.json" && { echo "ready at $(date -u +%Y-%m-%dT%H:%M:%SZ)"; exit 0; }
  sleep 600; n=$((n+1))
done
echo "not ready after ${1:-8} hours"; exit 2
