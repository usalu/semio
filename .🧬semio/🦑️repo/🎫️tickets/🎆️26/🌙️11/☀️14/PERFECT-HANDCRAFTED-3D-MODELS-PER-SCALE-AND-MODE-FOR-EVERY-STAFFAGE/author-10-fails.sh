#!/bin/sh
cd "$(dirname "$0")"; . ./author-10-env.sh
$BUN volumes-check.ts humans/$1 --quiet --json 2>/dev/null | python3 -c "
import json,sys
d=json.load(sys.stdin)
for m in d['measurements']:
    if not m['ok'] and not m['name'].startswith('extentDiff') and not m['name'].startswith('areaDev'): print(m['rule'],m.get('scale'),m['name'],round(m['value'],4),m['limit'],(m.get('note') or '')[:150])
"
