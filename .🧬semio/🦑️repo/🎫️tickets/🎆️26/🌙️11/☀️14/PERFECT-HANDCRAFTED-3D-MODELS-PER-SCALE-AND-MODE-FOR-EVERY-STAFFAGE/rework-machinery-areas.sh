#!/bin/sh
# 📐 Outer and exposed thermal areas per region as the product measures them: `sh rework-machinery-areas.sh <category>/<slug> <scale>`.
cd "$(dirname "$0")" || exit 2
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
"$BUN" volumes-check.ts "$1" --scale "$2" --rules volumes-thermal-coverage --product --json 2>&1 | python3 -c "
import sys,json
t=sys.stdin.read(); j=json.loads(t[t.index('{'):])
o={}
for m in j['measurements']:
    n=m['name']
    if 'rea' in n or not m['ok']: o[n.replace('info:','')]=round(m['value'],4)
print($2,' '.join(f'{k}={v}' for k,v in o.items()))
for f in j['findings'][:6]: print('  F',json.dumps(f)[:300])
"
