#!/bin/sh
# usage: spin-watch.sh <wrapper-sh-pid> <out-prefix> ; waits (bounded) for a >90% CPU descendant of the wrapper that stays hot, then snapshots it
cd /Users/ueli/Documents/semio || exit 1
wrapper="$1"; out="$2"; hot=0
for i in $(seq 1 3000); do
  kill -0 "$wrapper" 2>/dev/null || { echo "wrapper gone" >> "$out.txt"; exit 0; }
  for p in $(ps -axo pid,ppid | awk -v w="$wrapper" '{pp[$1]=$2} END{for(p in pp){q=p;while(q>1){if(q==w){print p;break};q=pp[q]}}}'); do
    c=$(ps -o pcpu= -p "$p" 2>/dev/null | awk '{print int($1)}')
    if [ "${c:-0}" -ge 90 ]; then hot=$((hot+1)); else hot=0; fi
    if [ "$hot" -ge 4 ]; then
      { date; echo "hot pid $p"; ps -axo pid,ppid,pcpu,stat,etime,command | awk '$3>20' | cut -c1-200; echo '--- lsof tcp'; lsof -nP -a -p "$p" -iTCP 2>/dev/null; echo '--- all nx daemons'; ps -axo pid,ppid,pcpu,etime,command | /usr/bin/grep -E "daemon/server/start|nx.*daemon" | /usr/bin/grep -v grep | cut -c1-200; } >> "$out.txt" 2>&1
      for f in $(find .cache/b/n -name '*.log' -mmin -40 2>/dev/null | head -8); do echo "=== $f" >> "$out.txt"; tail -40 "$f" >> "$out.txt" 2>&1; done
      kill -USR2 "$p"; sleep 8; kill -USR2 "$p"; sleep 2; sample "$p" 3 -file "$out.sample.txt" >/dev/null 2>&1
       echo "snapshot done $p" >> "$out.txt"; exit 0
    fi
  done
  sleep 5
done
