#!/bin/sh
# usage: dog.sh WRAPPER_PID LOG BINPATTERN STALL_SECONDS ; kills a test binary whose log has not grown for STALL seconds
W=$1; L=$2; P=$3; S=${4:-180}
last=0; since=$(date +%s)
while kill -0 "$W" 2>/dev/null; do
  sz=$(wc -c < "$L" 2>/dev/null || echo 0)
  now=$(date +%s)
  if [ "$sz" != "$last" ]; then last=$sz; since=$now; fi
  b=$(ps -ax -o pid,command | /usr/bin/grep -E "$P" | /usr/bin/grep -v -E "grep|dog.sh|bun repo" | awk '{print $1}')
  if [ -n "$b" ] && [ $((now-since)) -gt "$S" ]; then echo "[dog] stall $((now-since))s kill $b" >> "$L"; kill $b; since=$now; fi
  sleep 10
done
