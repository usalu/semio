#!/bin/bash
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17"
B=/Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
O="$T/STAFFAGE-CATALOG-SILHOUETTE-GUARD-PASSES-FOR-EVERY-ASSET/🗑️generated/fo-tri.jsonl"
cd "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript"
$B "$T/STAFFAGE-WEBSITE-CDN-REDEPLOY-READINESS/silhouette-triage.ts" current "$O" "$@" >/dev/null 2>&1
python3 - "$O" <<'P'
import json,sys
for l in open(sys.argv[1]):
    d=json.loads(l); print(d['asset'],len(d['fails']))
    for f in d['fails']: print('   ',{k:v for k,v in f.items() if k not in('section',)})
P
