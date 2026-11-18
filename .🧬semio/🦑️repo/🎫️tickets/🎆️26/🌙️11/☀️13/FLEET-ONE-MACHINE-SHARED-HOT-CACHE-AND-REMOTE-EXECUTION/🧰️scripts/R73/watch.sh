#!/bin/zsh
S=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/🧰️scripts
B=/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
L=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R73/watch.log
for r in 1 2 3 4 5; do
 sleep 170
 { date -u +"== %H:%M:%SZ"; cd $S/R73/proof; N=60 env -u SEMIO_FLEET_GROUP $B fresh.mjs 4d9db5baac29 2>&1 | grep -a "FRESH sample"
 python3 -I $S/R68/proof/watch.py 2>&1 | grep -a "ownersPerMinute" | python3 -c "
import sys,re
t=0;f=0
for l in sys.stdin:
  m=re.search(r\"'ownersPerMinute': ([0-9.]+)\",l);t+=float(m.group(1)) if m else 0
  m=re.search(r\"'failed': (\d+)\",l);f+=int(m.group(1)) if m else 0
print('pool owners/min',round(t,1),'failed',f)"
 python3 -I $S/R54/ready-table.py 2>&1 | cut -c1-110; } >> $L 2>&1
done
echo DONE >> $L
