#!/bin/sh
out="$1"; : > "$out"
while :; do
 t=$(date +%s)
 ps -axo pid=,ppid=,rss=,command= | awk -v t="$t" '{c=$4; n=split(c,a,"/"); b=a[n]; rss[$1]=$3; print t, $3/1024, b, $0 > "/dev/null"} {k=$4; sub(/.*\//,"",k); sum[k]+=$3} END{for(k in sum) if(sum[k]>300000) printf "%s %s %d\n", t, k, sum[k]/1024}' >> "$out"
 sleep 10
done
