#!/bin/sh
# 🛡️ Runs a command and stops it when its process tree holds more resident memory than its reservation: guarded.sh <limit MB> <log> <command and args…>
# Every 3 s the resident sets of the command and of all its descendants are summed; above the limit the tree is killed and the log says so. The log ends with the highest sum seen, the seconds and the exit code.
# For runs inside a capacity grant whose need is not measured yet, so that an underestimate cannot take memory the grant did not reserve.
set -u
limit=$1; log=$2; shift 2
tree() { echo "$1"; for child in $(pgrep -P "$1" 2>/dev/null); do tree "$child"; done; }
started=$(date +%s)
"$@" > "$log" 2>&1 &
root=$!
peak=0
while kill -0 "$root" 2>/dev/null; do
  sum=0
  for pid in $(tree "$root"); do
    rss=$(ps -o rss= -p "$pid" 2>/dev/null | tr -d ' ')
    [ -n "$rss" ] && sum=$(( sum + rss ))
  done
  [ "$sum" -gt "$peak" ] && peak=$sum
  if [ "$sum" -gt $(( limit * 1024 )) ]; then
    echo "[DEBUG] stopped: the process tree held $(( sum / 1024 )) MB resident, above the limit of $limit MB" >> "$log"
    for pid in $(tree "$root"); do kill -9 "$pid" 2>/dev/null; done
  fi
  sleep 3
done
wait "$root"
code=$?
echo "guarded: peak of the process tree $(( peak / 1024 )) MB resident (sampled every 3 s), $(( $(date +%s) - started )) s, exit $code" >> "$log"
tail -1 "$log"
