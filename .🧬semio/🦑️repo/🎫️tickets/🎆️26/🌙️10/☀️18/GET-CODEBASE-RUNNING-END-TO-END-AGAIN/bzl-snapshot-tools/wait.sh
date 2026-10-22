#!/bin/sh
# usage: wait.sh <pid> <log> <seconds> [marker-file-glob-dir]
pid=$1; log=$2; secs=$3
i=0
while [ $i -lt $((secs/5)) ]; do
  kill -0 $pid 2>/dev/null || { echo EXITED; break; }
  sleep 5; i=$((i+1))
done
grep -v -E "script\]|capacity|Store dependency concurrency|Another process is building|repository:command\] running" "$log" | tail -${4:-12} | cut -c1-400
kill -0 $pid 2>/dev/null && echo RUNNING
ls -lt /Users/ueli/Documents/semio/.cache/b/p/m/bazel/workspaces/*/*/snapshot.json 2>/dev/null | head -3
date
