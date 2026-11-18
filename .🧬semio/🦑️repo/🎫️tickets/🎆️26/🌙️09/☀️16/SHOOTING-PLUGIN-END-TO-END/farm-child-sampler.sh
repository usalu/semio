#!/bin/sh
out="$1"; : >> "$out"
while :; do
 t=$(date +%s)
 ps -axo pid=,ppid=,rss=,command= | LC_ALL=C awk '
  {pid=$1;ppid[$1]=$2;rss[$1]=$3;cmd[$1]=$0}
  /slice-pool/ && /worker\.ts/ {root[$1]=1}
  END{for(p in ppid){q=p;d=0;while(q in ppid && d<6){if(q in root && q!=p){print p,ppid[p],int(rss[p]/1024);break};q=ppid[q];d++}}}' |
 while read pid ppid mb; do
  fp=$(footprint -p "$pid" 2>/dev/null | sed -n 's/.*Footprint: \([0-9.]*\) \([KMG]B\).*/\1 \2/p' | head -1)
  echo "$t pid=$pid ppid=$ppid rssMB=$mb footprint=$fp"  >> "$out"
 done
 sleep 1
done
