#!/bin/zsh
# Samples the os-hub listening on <port> every <interval> s with macOS `sample` (3 s each), keeping the last
# <keep> captures under .🧬semio/🌐hub/s12-h9-logs/stall-<tag>/, until the hub, once seen, is gone for 120 s.
# usage: hub-stall-sampler.sh <port> <tag> [interval=8] [keep=10]
PORT=$1; TAG=$2; INTERVAL=${3:-8}; KEEP=${4:-10}
OUT="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s12-h9-logs/stall-$TAG"
mkdir -p "$OUT"
gone=0; n=0; seen=0
while [ $gone -lt 120 ]; do
  pid=$(lsof -nP -iTCP:$PORT -sTCP:LISTEN -t 2>/dev/null | head -1)
  if [ -z "$pid" ]; then [ $seen -eq 1 ] && gone=$((gone + INTERVAL)); sleep $INTERVAL; continue; fi
  seen=1
  gone=0; n=$((n + 1))
  stamp=$(date +%H%M%S)
  sample "$pid" 3 -file "$OUT/sample-$stamp.txt" >/dev/null 2>&1
  pg=$(docker ps --format '{{.Names}}' 2>/dev/null | /usr/bin/grep -m1 '^semio-hub-e2e-postgres-')
  [ -n "$pg" ] && docker exec "$pg" psql --username=semio --dbname=semio --command "SELECT a.pid, a.application_name, a.state, a.wait_event_type, a.wait_event, now() - a.xact_start AS xact_age, now() - a.query_start AS query_age, left(a.query, 160) AS query, (SELECT string_agg(l.locktype || ':' || l.mode || ':' || l.granted, ',') FROM pg_locks l WHERE l.pid = a.pid) AS locks FROM pg_stat_activity a WHERE a.datname = 'semio' ORDER BY a.pid" > "$OUT/pg-$stamp.txt" 2>&1
  ls -t "$OUT"/sample-*.txt 2>/dev/null | tail -n +$((KEEP + 1)) | while read f; do rm -f "$f"; done
  ls -t "$OUT"/pg-*.txt 2>/dev/null | tail -n +$((KEEP * 3 + 1)) | while read f; do rm -f "$f"; done
  sleep $INTERVAL
done
echo "sampler done $(date +%T) captures=$n" > "$OUT/done.txt"
