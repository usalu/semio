#!/usr/bin/env bash
# usage: rss-run-summary.sh <label>   wall seconds, peak RSS, project-stage window and governor waits of one harness run
G="$(cd "$(dirname "$0")/.." && pwd)/🗑️generated/rss-profile"; l="$1"
wall=$(grep -ao '^\[t=[0-9]*\]' "$G/$l.log" | tail -1 | tr -dc 0-9)
first=$(grep -a "Inferring projects: 0 of" "$G/$l.log" | head -1 | grep -o 't=[0-9]*' | tr -dc 0-9)
last=$(grep -a "Inferring projects: 40 of" "$G/$l.log" | head -1 | grep -o 't=[0-9]*' | tr -dc 0-9)
echo "$l wall=${wall}s projectsStage=$((last-first))s peakRss=$(grep -a peakMb "$G/$l.json" | tr -dc 0-9)MB waits=$(grep -ac 'waiting for memory' "$G/$l.log") peakHeap=$(python3 "$(dirname "$0")/rss-stage-summary.py" "$G/$l.log" | grep -o 'peak_heap= *[0-9]*' | tr -dc '0-9\n' | sort -n | tail -1)"
