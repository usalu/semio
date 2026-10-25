#!/bin/sh
# usage: nx-fail-watch.sh <wrapper-pid> <run-log> <out-prefix>: when the run log shows an Nx failure, copy the daemon/nx logs that are still mounted
cd /Users/ueli/Documents/semio || exit 1
wrapper="$1"; log="$2"; out="$3"
for i in $(seq 1 4000); do
  kill -0 "$wrapper" 2>/dev/null || exit 0
  if grep -q "^ NX " "$log" 2>/dev/null; then
    sleep 3
    { date; for f in $(find .cache/b/n -name '*.log' -mmin -90 2>/dev/null | head -12); do echo "=== $f"; tail -80 "$f"; done; ps -axo pid,ppid,pcpu,etime,command | grep -E 'daemon/server/start' | grep -v grep | cut -c1-200; } > "$out.txt" 2>&1
    exit 0
  fi
  sleep 3
done
