#!/bin/sh
# usage: watch-builder.sh <pid> <build log> <out>; read-only: samples the phase line and rss of the graph builder every 15 s until it exits
pid=$1; log=$2; out=$3
while kill -0 "$pid" 2>/dev/null; do
  line=$(grep -a -E 'Inferring projects|graph: (refreshing|published|[0-9]+ stale|inferring|asking)' "$log" | tail -1 | cut -c1-140)
  echo "$(date +%H:%M:%S) rss=$(ps -o rss= -p $pid | tr -d ' ') cpu=$(ps -o pcpu= -p $pid | tr -d ' ') $line" >> "$out"
  sleep 15
done
echo "$(date +%H:%M:%S) exited" >> "$out"
