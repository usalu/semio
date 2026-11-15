#!/bin/bash
# usage: author-69-areas.sh slug [scale]: outer area per region (product) against the declared areaM2, and the total deviation
cd "$(dirname "$0")"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
$BUN volumes-check.ts humans/$1 --product --rules volumes-thermal-coverage ${2:+--scale $2} 2>&1 | python3 -c "
import sys,re,json
slug='$1'
a=json.load(open('/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/%s/🔣️.json'%slug))['asset']
decl={r['id']:(r['areaM2'],r.get('areaBasis'),r.get('model')) for r in a['thermal']['regions']}
for l in sys.stdin:
    if 'thermal-coverage' not in l: continue
    sc=l.split()[2]
    out=dict(re.findall(r'outerAreaM2:([a-z0-9-]+)=([0-9.]+)',l))
    ex=dict(re.findall(r'exposedAreaM2:([a-z0-9-]+)=([0-9.]+)',l))
    tot=re.findall(r'areaDeviationTotal=([0-9.]+)',l)
    ms=0;ds=0
    print(sc,'total dev',tot)
    for k,v in out.items():
        d=decl.get(k,(None,))
        print('  %-18s outer %.3f declared %s dev %+.0f%%  %s'%(k,float(v),d[0],(float(v)/d[0]-1)*100 if d[0] else 0, d[1] or ''))
"
