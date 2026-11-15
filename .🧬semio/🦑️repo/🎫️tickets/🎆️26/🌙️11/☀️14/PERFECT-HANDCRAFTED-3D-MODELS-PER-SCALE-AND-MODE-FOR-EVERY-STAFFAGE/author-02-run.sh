#!/bin/bash
# usage: author-02-run.sh <slug> [scale]   -> generate, check (condensed)
cd "$(dirname "$0")"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
slug=$1; shift
$BUN author-02-$slug.ts "$@" 2>&1 | tail -3
$BUN volumes-check.ts vegetation/$slug --json 2>/dev/null | python3 -c "
import json,sys
t=sys.stdin.read(); d=json.loads(t[t.index('{"asset"') if '{"asset"' in t else t.index('{'):])
bad={}
for m in d['measurements']:
    if not m['ok']:
        key=(m['scale'] if 'scale' in m else 0, m['rule'])
        bad.setdefault(key,[]).append('%s=%.4g(%s%.4g)'%(m['name'],m['value'],'>=' if m['bound']=='min' else '<=',m['limit']))
for k in sorted(bad): print('1:%s'%k[0],k[1].replace('volumes-',''),' '.join(bad[k])[:420])
print('FAILS',sum(len(v) for v in bad.values()))
for f in d['findings'][:0]: print(f)
"
