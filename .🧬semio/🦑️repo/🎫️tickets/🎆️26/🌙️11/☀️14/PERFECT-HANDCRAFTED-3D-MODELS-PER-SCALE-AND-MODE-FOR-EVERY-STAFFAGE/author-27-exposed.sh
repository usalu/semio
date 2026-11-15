#!/bin/zsh
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
slug=$1; src=$2; s=$3
$BUN volumes-check.ts humans/$slug --scale $s --rules volumes-thermal-coverage --owned --source $src --json 2>&1 | python3 -c "
import sys,json,re
t=sys.stdin.read(); d=json.loads(t[t.index('{'):])
for m in d['measurements']:
    if m['name'].startswith('areaDeviation'):
        n=m.get('note','') or ''
        mm=re.search(r'exposed ([0-9.]+) m2 of ([0-9.]+) m2 mesh, areaM2 ([0-9.]+)',n)
        print(m['name'].split(':')[1].ljust(16), 'dev',round(m['value'],3), ('exposed %s mesh %s declared %s'%mm.groups()) if mm else n[:90])
"
