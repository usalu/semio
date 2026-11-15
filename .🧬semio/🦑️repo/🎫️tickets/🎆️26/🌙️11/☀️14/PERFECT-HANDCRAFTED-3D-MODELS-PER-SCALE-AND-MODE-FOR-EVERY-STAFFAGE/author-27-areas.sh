#!/bin/zsh
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
slug=$1; src=$2; s=$3
$BUN volumes-check.ts humans/$slug --scale $s --rules volumes-thermal-coverage --source $src --json 2>&1 | python3 -c "
import sys,json
t=sys.stdin.read(); d=json.loads(t[t.index('{'):])
for m in d['measurements']:
    if m['name'].startswith('areaDeviation') or 'Total' in m['name']: print(m['name'],round(m['value'],3),m.get('note','')[:200])
"
