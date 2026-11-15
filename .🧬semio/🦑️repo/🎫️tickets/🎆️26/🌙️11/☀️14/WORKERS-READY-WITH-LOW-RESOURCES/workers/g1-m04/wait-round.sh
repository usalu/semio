#!/bin/bash
# ⏳ Waits for a round above <reported>: looks every 60 s at workers/g1-m04/next.md, else ROUND.md, for at most 6 hours. Usage: wait-round.sh <reported>
W="$(cd "$(dirname "$0")/../.." && pwd)"; M="$W/workers/g1-m04"; reported="$1"; n=0
while [ $n -lt 360 ]; do
  file="$W/ROUND.md"; [ -f "$M/next.md" ] && file="$M/next.md"
  round="$(grep -m1 -Eo '^round: *[0-9]+' "$file" 2>/dev/null | grep -Eo '[0-9]+')"
  if [ -n "$round" ] && [ "$round" -gt "$reported" ] && [ ! -f "$M/round-$round.md" ]; then echo "round $round in $(basename "$file") at $(date -u +%Y-%m-%dT%H:%M:%SZ)"; exit 0; fi
  sleep 60; n=$((n+1))
done
echo "no higher round after 6 hours"; exit 2
