#!/bin/sh
# usage: footprint-sampler.sh <pid-of-wrapper> <out.tsv> [interval]   samples the physical footprint (MB) of the newest bun child of the wrapper tree every interval seconds until the wrapper exits
W=$1; OUT=$2; I=${3:-10}
echo "t\tpid\tfootprintMB\trssMB" >"$OUT"; t=0
while kill -0 "$W" 2>/dev/null; do
  for p in $(pgrep -P "$W"; pgrep -P "$(pgrep -P "$W" | head -1)" 2>/dev/null); do
    fp=$(footprint -p "$p" 2>/dev/null | awk '/Footprint:/{v=$2; u=$3; if(u=="GB")v=v*1024; if(u=="KB")v=v/1024; print int(v); exit}')
    rss=$(ps -o rss= -p "$p" | awk '{print int($1/1024)}')
    [ -n "$fp" ] && echo "$t\t$p\t$fp\t$rss" >>"$OUT"
  done
  sleep "$I"; t=$((t+I))
done
