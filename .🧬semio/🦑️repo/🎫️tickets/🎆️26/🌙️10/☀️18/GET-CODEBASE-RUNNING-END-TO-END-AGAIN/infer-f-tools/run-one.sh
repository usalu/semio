#!/bin/sh
# usage: run-one.sh <label> <root> [ramBytes] [capKB]; runs infer-one.mjs detached, bounded, kills only its own pid at the RSS cap
cd /Users/ueli/Documents/semio || exit 1
C='.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN'
BUN=.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
label=$1; root=$2; ram=${3:-3221225472}; cap=${4:-4200000}
log="$C/🗑️generated/infer-f/one-$label.log"; rm -f "$log"
pid=$(env -u SEMIO_SCRIPT_STALL_MS BUN_JSC_forceRAMSize=$ram python3 "$C/chain-tools/detach.py" "$log" nice -n 10 $BUN "$C/infer-f-tools/infer-one.mjs" "$root" "$C/🗑️generated/infer-f/$label.json")
echo "pid $pid $(ps -o lstart= -p $pid)"
i=0; while [ $i -lt 85 ]; do kill -0 $pid 2>/dev/null || break; r=$(ps -o rss= -p $pid); if [ "${r:-0}" -gt "$cap" ]; then echo "RSS cap hit $r: killing own pid $pid"; kill $pid; break; fi; sleep 6; i=$((i+1)); done
kill -0 $pid 2>/dev/null && echo "still running pid $pid"
grep -a '\[DEBUG\]' "$log" | tail -3; tail -c 500 "$log" | grep -a '^{' 
