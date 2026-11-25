#!/bin/zsh
# 🚦️ W21: waits until the 1-minute load is below 30 (polls every 60 s), then runs one test file through `repo test-file` into 🗑️generated/<name>.txt and prints the summary lines. Usage: run-gated.sh <name> <test file relative to the checkout>
name=$1; file=$2; out="${0:A:h}/🗑️generated/$name.txt"
while true; do l=$(uptime | sed -E 's/.*averages?: ([0-9.]+).*/\1/'); awk -v l=$l 'BEGIN{exit !(l<30)}' && break; echo "$(date +%T) load $l, waiting"; sleep 60; done
echo "$(date +%T) load $l, running $name"
cd /Users/ueli/Documents/semio && repo test-file "$file" > "$out" 2>&1; code=$?
grep -E "^\(fail\)|^ *[0-9]+ (pass|fail)$|^test-file:|did not settle" "$out" | cut -c1-300
echo "exit $code $(uptime)"
