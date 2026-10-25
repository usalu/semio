#!/bin/sh
# usage: sample.sh <runpid> <log>   samples bun children of runpid + graph leases every 10 s until runpid exits
pid=$1; log=$2
while kill -0 "$pid" 2>/dev/null; do
  echo "=== $(date +%H:%M:%S)" >>"$log"
  ps -axo pid,ppid,rss,etime,command | awk -v p="$pid" 'BEGIN{} {a[$1]=$2; l[$1]=$0} END{for(i in a){x=i;while(x in a&&x!=p&&x!=1)x=a[x]; if(x==p)print substr(l[i],1,150)}}' >>"$log"
  (cd /Users/ueli/Documents/semio && nice -n 10 bun repo leases status 2>&1 | /usr/bin/grep -E "repository-graph-facts|build-graph-state" | cut -c1-200) >>"$log"
  sleep 10
done
echo "=== exit $(date +%H:%M:%S)" >>"$log"
