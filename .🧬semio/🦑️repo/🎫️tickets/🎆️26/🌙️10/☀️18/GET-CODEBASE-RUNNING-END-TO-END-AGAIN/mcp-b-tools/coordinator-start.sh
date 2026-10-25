#!/bin/sh
cd /Users/ueli/Documents/semio || exit 1
G=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/mcp-b"
nice -n 10 repo --tool=bazel run @semio-tech/repo-coordinator:start -- --port 6199 > "$G/coordinator-start.log" 2>&1 &
pid=$!
echo $pid > "$G/coordinator-start.pid"
ok=no
for i in $(seq 1 360); do
  code=$(curl -s -m 2 -o /dev/null -w "%{http_code}" http://127.0.0.1:6199/api/v1/health) && [ "$code" = "200" ] && { ok=yes; break; }
  kill -0 $pid 2>/dev/null || break
  sleep 10
done
body=$(curl -s -m 3 http://127.0.0.1:6199/api/v1/health)
echo "health ok=$ok body=$body" > "$G/coordinator-start.out"
kill -TERM $pid 2>/dev/null; sleep 5; kill -0 $pid 2>/dev/null && kill -KILL $pid 2>/dev/null
exit 0
