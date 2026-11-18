export B=/Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
export T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17"
export OLD="$T/STAFFAGE-WEBSITE-CDN-REDEPLOY-READINESS"
export NEW="$T/STAFFAGE-CATALOG-SILHOUETTE-GUARD-PASSES-FOR-EVERY-ASSET"
export G="$NEW/🗑️generated"
export PK="/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript"
tri() { (cd "$PK" && $B "$OLD/silhouette-triage.ts" current "$G/$1.jsonl" "${@:2}" >/dev/null 2>&1; python3 -c "
import json,sys
for l in open('$G/$1.jsonl'):
    d=json.loads(l); print(d['asset'],len(d.get('fails',[])),d.get('error',''))
    for f in d.get('fails',[]): print('  ',f)
"); }
diag() { (cd "$PK" && $B "$OLD/silhouette-diag.ts" "$@"); }
blobs() { (cd "$PK" && $B "$NEW/machinery-blobs.ts" "$@" 2>&1); }
