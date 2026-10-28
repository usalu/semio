#!/bin/sh
# [gram-g] one official graph-loading command (`repo leases status`) with a read-only RSS sampler of its whole process tree; kills ONLY its own tree above the cap. usage: official-watch.sh <label> <capMB>
cd /Users/ueli/Documents/semio || exit 1
C='.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN'
BUN=.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
label=$1; cap=$2; shift; shift; out="$C/🗑️generated/gram-g/$label"
env -u SEMIO_SCRIPT_STALL_MS nice -n 10 $BUN repo "$@" > "$out.log" 2>&1 &
root=$!
echo "root $root $(ps -o lstart= -p $root)" > "$out.rss"
peak=0
while kill -0 $root 2>/dev/null; do
  tree=$(ps -Ao pid,ppid,rss | awk -v r=$root 'BEGIN{k[r]=1} {p[$1]=$2; m[$1]=$3} END{for(i=0;i<8;i++)for(x in p)if(k[p[x]])k[x]=1; s=0; for(x in k)s+=m[x]; print int(s/1024)}')
  [ "$tree" -gt "$peak" ] && peak=$tree
  echo "$(date +%T) tree ${tree} MB peak ${peak} MB" >> "$out.rss"
  ps -Ao pid,ppid,rss,command | awk -v r=$root 'BEGIN{k[r]=1} {p[$1]=$2; m[$1]=$3; c[$1]=substr($0,index($0,$4),90)} END{for(i=0;i<8;i++)for(x in p)if(k[p[x]])k[x]=1; for(x in k)if(m[x]>200000)printf "    pid %s rss %dMB %s\n",x,m[x]/1024,c[x]}' >> "$out.rss"
  if [ "$tree" -gt "$cap" ]; then echo "CAP hit: killing own tree $root" >> "$out.rss"; pkids=$(ps -Ao pid,ppid | awk -v r=$root '$2==r{print $1}'); kill $pkids $root 2>/dev/null; break; fi
  sleep 5
done
echo "done peak ${peak} MB" >> "$out.rss"
