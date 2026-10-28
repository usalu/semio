#!/bin/sh
# usage: rss-guard.sh <limit-MB> <command...> : runs the command and kills ONLY that command's process group when its tree RSS passes the limit
limit=$1; shift
"$@" &
pid=$!
while kill -0 $pid 2>/dev/null; do
  rss=$(ps -o rss= -p $pid 2>/dev/null | awk '{s+=$1} END {print s/1024}')
  kids=$(pgrep -P $pid 2>/dev/null | xargs -I{} ps -o rss= -p {} 2>/dev/null | awk '{s+=$1} END {print s/1024}')
  total=$(echo "${rss:-0} ${kids:-0}" | awk '{print int($1+$2)}')
  if [ "$total" -gt "$limit" ]; then echo "rss-guard: $total MB > $limit MB, killing $pid" >&2; kill $pid; wait $pid 2>/dev/null; exit 137; fi
  sleep 0.5
done
wait $pid
