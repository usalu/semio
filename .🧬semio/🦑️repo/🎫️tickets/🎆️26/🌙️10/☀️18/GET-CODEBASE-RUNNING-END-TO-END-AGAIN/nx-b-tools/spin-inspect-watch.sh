#!/bin/sh
# usage: spin-inspect-watch.sh <wrapper-pid> <out-prefix>: for a run started with BUN_INSPECT=ws://127.0.0.1:0/nxb, introspects the first hot descendant through its inspector
cd /Users/ueli/Documents/semio || exit 1
wrapper="$1"; out="$2"; hot=0; B=/Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/nx-b-tools/inspect-spin.ts"
for i in $(seq 1 6000); do
  kill -0 "$wrapper" 2>/dev/null || exit 0
  for p in $(ps -axo pid,ppid | awk -v w="$wrapper" '{pp[$1]=$2} END{for(p in pp){q=p;while(q>1){if(q==w){print p;break};q=pp[q]}}}'); do
    c=$(ps -o pcpu= -p "$p" 2>/dev/null | awk '{print int($1)}')
    if [ "${c:-0}" -ge 80 ]; then hot=$((hot+1)); else hot=0; fi
    if [ "$hot" -ge 6 ]; then
      port=$(lsof -nP -a -p "$p" -iTCP -sTCP:LISTEN 2>/dev/null | awk 'NR>1{n=split($9,a,":");print a[n];exit}')
      { date; ps -o pid,ppid,pcpu,lstart,command -p "$p" | cut -c1-200; echo "port=$port"; lsof -nP -p "$p" 2>/dev/null | awk '{print $4,$5,$9}' | grep -v 'claims/' | head -40; } > "$out.txt" 2>&1
      [ -n "$port" ] && "$B" "$T" "$port" nxb >> "$out.txt" 2>&1
      sample "$p" 3 -file "$out.sample.txt" >/dev/null 2>&1
      exit 0
    fi
  done
  sleep 5
done
