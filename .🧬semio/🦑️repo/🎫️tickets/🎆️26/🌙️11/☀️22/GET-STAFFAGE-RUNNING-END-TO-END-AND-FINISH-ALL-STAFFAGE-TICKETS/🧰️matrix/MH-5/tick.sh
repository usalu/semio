#!/bin/zsh
# ⏱️ One short line of progress after waiting up to 170 s for a change: tick.sh prints the time, the number of result lines of the landings and searches, and the newest result line.
export BATCH=MH-5
source "${0:A:h}/../MR/env.sh"
n() { cat "$GEN"/land/*.out "$GEN"/auto/*.log 2>/dev/null | grep -cE '^STOP|^REFUSED|^gate: |^gate exit|^matrix exit|^check: |^KEPT|^OPEN| done: '; }
start=$(n); i=0
while [[ "$(n)" == "$start" && $i -lt 34 ]]; do sleep 5; i=$((i + 1)); done
echo "$(date +%H:%M) results $(n) (was $start); newest: $(ls -t "$GEN"/land/*.out "$GEN"/auto/*.log | head -1 | xargs grep -hE '^STOP|^REFUSED|^gate: |^gate exit|^matrix exit|^check: |^KEPT|^OPEN| done: ' | tail -1 | cut -c1-150)"
