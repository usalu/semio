#!/bin/bash
# usage: census.sh crate-name...  (source-boundary gate measure of the named roots + closure; prints only over/near rows)
cd /Users/ueli/Documents/semio
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
printf '%s\n' "$@" > "$C/🗑️generated/1029/energy-roots.txt"
bun "$C/1028-tools/gatecensus/census.ts" "$C/🗑️generated/1029/energy-roots.txt" > "$C/🗑️generated/1029/energy-census.json" 2>/dev/null
python3 - <<'PY'
import json
C='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS/🗑️generated/1029/'
d=json.loads(open(C+'energy-census.json').read().strip().split('\n')[-1])
for r in d['rows']:
    o=r['over']
    if o['bytes']>-8192 or o['files']>-2 or o['lines']>-100:
        print(('OVER ' if max(o.values())>0 else 'near '),r['name'],r['bytes'],r['files'],r['lines'],o)
PY
