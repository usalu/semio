#!/bin/sh
cd "$(dirname "$0")"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
ASSET=$1; shift
for n in "$@"; do
echo "== $n"
$BUN volumes-check.ts $ASSET --product --scale $n --rules volumes-thermal-coverage --json 2>/dev/null | python3 -c "
import json,sys
d=json.load(sys.stdin)
for m in d['measurements']:
    if 'area' in m['name']: print(m['name'],round(m['value'],3),m.get('note','')[:300])"
done
