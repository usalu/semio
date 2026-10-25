#!/bin/sh
# usage: watch-rss.sh <port> <limit-mb> <max-seconds> <log>
port=$1; limit=$2; max=$3; log=$4
t0=$(date +%s)
pid=""
while :; do
  now=$(date +%s)
  [ $((now - t0)) -gt "$max" ] && echo "[watch] deadline" >>"$log" && break
  pid=$(lsof -nP -iTCP:"$port" -sTCP:LISTEN -t 2>/dev/null | head -1)
  if [ -n "$pid" ]; then
    rss=$(ps -o rss= -p "$pid" | tr -d ' ')
    echo "$(date +%T) pid=$pid lstart='$(ps -o lstart= -p "$pid")' rss_mb=$((rss / 1024)) cpu=$(ps -o pcpu= -p "$pid")" >>"$log"
    if [ $((rss / 1024)) -gt "$limit" ]; then echo "[watch] LIMIT -> SIGINT then TERM then KILL $pid" >>"$log"; kill -INT "$pid"; sleep 60; kill -0 "$pid" 2>/dev/null && kill "$pid"; sleep 20; kill -0 "$pid" 2>/dev/null && kill -9 "$pid"; break; fi
  fi
  sleep 10
done
