#!/bin/zsh
# 📏️ H10: samples the os-hub listening on <port> every <interval> s (ps RSS + footprint phys_footprint) into <capture>.
# usage: rss-sampler.sh <port> <capture> [interval]
PORT=$1; OUT=$2; IV=${3:-15}
while true; do
  pid=$(lsof -nP -iTCP:$PORT -sTCP:LISTEN -t 2>/dev/null | head -1)
  if [ -n "$pid" ]; then
    rss=$(ps -o rss= -p $pid | tr -d ' ')
    fp=$(footprint $pid 2>/dev/null | /usr/bin/grep -m1 -i "footprint:" | tr -s ' ')
    echo "$(date +%T) pid=$pid rssKiB=$rss $fp" >> "$OUT"
  fi
  sleep $IV
done
