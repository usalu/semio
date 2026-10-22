#!/bin/sh
# usage: wait.sh <log> : waits up to ~9 min for <log>.status, then prints status + test summary lines
cd /Users/ueli/Documents/semio || exit 2
for i in $(seq 1 54); do [ -f "$1.status" ] && break; sleep 10; done
cat "$1.status" 2>&1
/usr/bin/grep -E ': error|^error|panicked|test result|FAILED|failed|left:|right:|Running |^---- |could not compile' "$1" | /usr/bin/grep -v warning | cut -c1-400 | tail -${2:-25}
