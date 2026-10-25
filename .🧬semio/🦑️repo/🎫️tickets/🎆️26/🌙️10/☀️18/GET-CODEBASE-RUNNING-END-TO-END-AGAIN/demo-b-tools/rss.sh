#!/bin/sh
# usage: rss.sh <wrapper-pid> <outfile> ; samples RSS(MB) of every descendant "bun --eval" of the wrapper until it exits
w=$1; o=$2
while kill -0 $w 2>/dev/null; do
 ps -axo pid,ppid,rss,command | awk -v w=$w '{pp[$1]=$2;rs[$1]=$3;cm[$1]=$0} END{for(p in pp){q=p;while(q>1&&q!=w)q=pp[q]; if(q==w&&cm[p]~/bun --eval/)printf "%d %d\n",p,rs[p]/1024}}' | while read p r; do echo "$(date +%H:%M:%S) $p $r" >> $o; done
 sleep 5
done
