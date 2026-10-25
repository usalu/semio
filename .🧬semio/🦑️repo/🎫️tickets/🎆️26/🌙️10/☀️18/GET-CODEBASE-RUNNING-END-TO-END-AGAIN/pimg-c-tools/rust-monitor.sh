#!/bin/sh
M="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/rust/fixed/m"
i=0
while [ $i -lt 45 ]; do
  F=$(df -k "$M" | tail -1 | awk '{print $4}')
  W=$(ps -axo pid,command | grep -E "semio-unit-cache.*--watch" | grep -v grep | awk '{print $1}' | tr '\n' ',')
  C=$(ps -axo command | grep -cE "(^|/)cargo( |$)|rustc " )
  S=$(wc -l < "$M/u/fill.jsonl" 2>/dev/null)
  echo "$(date +%H:%M:%S) freeKiB=$F freeGiB=$((F/1048576)) watcher=[$W] compilers=$C fillRows=$S"
  i=$((i+1)); sleep 60
done
echo DONE
