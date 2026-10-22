#!/bin/sh
# usage: update_locks.sh <list> <logdir>; sequential cargo update --offline per stale workspace lock
cd /Users/ueli/Documents/semio || exit 1
while IFS= read -r lock; do
  [ -z "$lock" ] && continue
  dir=$(dirname "$lock")
  [ -f "$dir/Cargo.toml" ] || { echo "no manifest $dir" >> "$2/status.txt"; continue; }
  n=0
  while :; do
    nice -n 10 bun repo cargo --store=shared --configuration=prototype -- update --workspace --offline --manifest-path "$dir/Cargo.toml" > "$2/last.log" 2>&1
    rc=$?
    if [ $rc -eq 0 ]; then echo "ok $dir" >> "$2/status.txt"; break; fi
    n=$((n+1))
    if [ $n -ge 3 ]; then echo "FAIL($rc) $dir" >> "$2/status.txt"; cp "$2/last.log" "$2/fail-$(echo "$dir" | tr '/' '_' | tail -c 60).log"; break; fi
    sleep 30
  done
done < "$1"
echo done >> "$2/status.txt"
