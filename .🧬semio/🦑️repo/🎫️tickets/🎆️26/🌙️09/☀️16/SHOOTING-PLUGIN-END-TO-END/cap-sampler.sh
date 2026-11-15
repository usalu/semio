#!/bin/sh
out="$1"; : > "$out"
while :; do
 t=$(date +%s)
 ps -axo pid=,ppid=,rss= | awk '$3>300000{print $1, $2, int($3/1024)}' | while read pid ppid mb; do
  cmd=$(ps -o command= -p "$pid" 2>/dev/null | cut -c1-60 | tr -c '[:print:]' '?')
  cap=$(ps eww -p "$pid" 2>/dev/null | tr ' ' '\n' | grep -E '^(BUN_JSC_forceRAMSize|MIMALLOC_PURGE_DELAY)=' | tr '\n' ' ')
  echo "$t pid=$pid ppid=$ppid mb=$mb cap=[$cap] $cmd" >> "$out"
 done
 sleep 5
done
