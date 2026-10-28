#!/bin/sh
# [gram2-g] read-only sampler of ONE own pid and its descendants: every N s "HH:MM:SS rss <MB> tree <MB> kids <n> fp <footprint> tag1 <dirty> <reclaimable>" until the pid ends. usage: fp.sh <pid> <out> [interval]
pid=$1; out=$2; every=${3:-10}
while kill -0 "$pid" 2>/dev/null; do
  rss=$(ps -o rss= -p "$pid" | awk '{print int($1/1024)}')
  tree=$(python3 "$(dirname "$0")/tree.py" "$pid")
  fp=$(footprint -p "$pid" 2>/dev/null | awk '/Footprint:/{print $5,$6} /App-Specific Tag 1/{print "tag1",$1,$2,$5,$6}' | tr '\n' ' ')
  echo "$(date +%T) rss $rss tree $tree fp $fp" >> "$out"
  sleep "$every"
done
