B=/Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
S=/private/tmp/claude-501/-Users-ueli-Documents-semio/d2e1de2c-c5c8-40c3-9aef-100ee29812ea/scratchpad/pg.jsonl
for ab in "$@"; do set -- $ab; python3 spread-pigeon-3.py $1 $2; echo "== $ab"; $B spread-sweep.ts pigeon | cut -c1-200; $B ../STAFFAGE-WEBSITE-CDN-REDEPLOY-READINESS/silhouette-triage.ts current $S pigeon; cut -c1-250 $S; done
