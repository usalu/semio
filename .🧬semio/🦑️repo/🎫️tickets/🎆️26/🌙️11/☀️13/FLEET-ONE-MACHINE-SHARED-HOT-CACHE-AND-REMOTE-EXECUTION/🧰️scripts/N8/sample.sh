#!/bin/bash
# Samples the memory footprint of a process tree every INTERVAL seconds until the root pid ends: sample.sh <pid> <output file> [interval]
root="$1"; out="$2"; interval="${3:-15}"
while kill -0 "$root" 2>/dev/null; do
  pids="$root $(pgrep -P "$root" | tr '\n' ' ')"
  all="$pids"; for p in $pids; do all="$all $(pgrep -P "$p" | tr '\n' ' ')"; done
  line="$(date +%H:%M:%S)"
  total=0
  for p in $all; do
    f=$(footprint "$p" 2>/dev/null | /usr/bin/grep -E "^ *Footprint:|Footprint:" | head -1 | /usr/bin/sed -E 's/.*Footprint: *//')
    line="$line $p=${f:-?}"
  done
  echo "$line" >> "$out"
  sleep "$interval"
done
