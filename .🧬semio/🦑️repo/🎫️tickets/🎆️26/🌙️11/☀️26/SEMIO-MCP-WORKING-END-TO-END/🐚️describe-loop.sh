#!/bin/bash
# 🔁️ Retries `describe --all`, keeps one log per attempt and kills an attempt whose log stays silent for STALL seconds.
cd /Users/ueli/Documents/semio || exit 1
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️26/SEMIO-MCP-WORKING-END-TO-END/🗑️generated"
STALL=${STALL:-1200}
tree() { local p=$1; for c in $(pgrep -P "$p"); do tree "$c"; done; echo "$p"; }
for i in $(seq 1 "${ATTEMPTS:-6}"); do
  log="$T/describe-all-v$i.txt"
  repo run workspace:components -- describe --all > "$log" 2>&1 &
  pid=$!
  while kill -0 "$pid" 2>/dev/null; do
    sleep 60
    age=$(( $(date +%s) - $(stat -f %m "$log") ))
    if [ "$age" -gt "$STALL" ]; then echo "attempt $i stalled ${age}s, killing"; kill -TERM $(tree "$pid") 2>/dev/null; sleep 15; kill -KILL $(tree "$pid") 2>/dev/null; fi
  done
  wait "$pid"; c=$?
  echo "attempt $i exit $c $(date +%H:%M) $(df -h / | tail -1 | awk '{print $4}')"
  [ "$c" = 0 ] && exit 0
  sed 's/\x1b\[[0-9;]*m//g' "$log" | grep -m3 '^[A-Za-z]*Error:\|^error\[\|^error:\|No space\|Export named\|Internal error' | cut -c1-300
  sleep 120
done
exit 1
