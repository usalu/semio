#!/bin/zsh
G="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/wave-1122/🗑️generated/E63"
n=0
end=$(( $(date +%s) + 600 ))
while [ $(date +%s) -lt $end ]; do
 pid=$(ps -eo pid,etime,command | awk '/👷️|slice/ && /bun/ && !/awk/ {print $1}' | head -0)
 p=$(python3 -c "
import json
try:
 d=json.load(open('/Users/ueli/Documents/semio/.cache/f/slice-memory.json'))
 print(d['pid'],int(d['uptimeSeconds']))
except Exception: print('0 0')")
 set -- $p
 if [ "$1" != "71335" ] && [ "$1" != "0" ] && [ "$2" -ge 15 ] && [ "$n" -lt 2 ] && [ "$1" != "$last" ]; then
   last=$1; n=$((n+1)); sample $1 6 -file "$G/stack-$1.txt" >/dev/null 2>&1
 fi
 sleep 3
done
