#!/bin/zsh
# Samples the resident memory of the one H9 e2e hub serving on the given port every 20 s (ps RSS + footprint).
# usage: rss-sampler.sh <port> <capture>
PORT=$1; OUT=$2
while true; do
  pid=$(lsof -nP -iTCP:$PORT -sTCP:LISTEN -t 2>/dev/null | head -1)
  if [ -n "$pid" ]; then
    rss=$(ps -o rss= -p $pid | tr -d ' ')
    fp=$(footprint $pid 2>/dev/null | /usr/bin/grep -m1 -i "footprint:" | tr -s ' ')
    echo "$(date +%T) pid=$pid rssKiB=$rss $fp" >> "$OUT"
  fi
  sleep 20
done
