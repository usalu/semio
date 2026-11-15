#!/bin/bash
# ⏳ Waits (60 s steps, at most the given number of steps) until next.md or ROUND.md names a round this worker has not reported yet. Usage: wait-round.sh <steps>
here=$(cd "$(dirname "$0")" && pwd); W=$(cd "$here/../.." && pwd); i=0
while :; do
  f="$here/next.md"; [ -f "$f" ] || f="$W/ROUND.md"
  n=$(grep -m1 -o -E '^round: *[0-9]+' "$f" 2>/dev/null | grep -o -E '[0-9]+')
  if [ -n "$n" ] && [ ! -f "$here/round-$n.md" ]; then echo "new round $n in $f"; exit 0; fi
  i=$((i+1)); [ "$i" -ge "$1" ] && { echo "no new round after $i checks (last: round ${n:-none})"; exit 0; }
  sleep 60
done
