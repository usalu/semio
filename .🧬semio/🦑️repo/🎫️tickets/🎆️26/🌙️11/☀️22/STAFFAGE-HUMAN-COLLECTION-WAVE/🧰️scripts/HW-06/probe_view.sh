#!/bin/sh
# 🔎️ Which view and pose make a shape fail print suitability: sh probe_view.sh <slug> <scale> <shape-id>
SLUG=$1; SC=$2; ID=$3
G="../../🗑️generated/HW-06"; B=/Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
for v in elevation front plan; do
python3 - "$SLUG" "$SC" "$ID" "$v" "$G/t.json" <<'PY'
import sys,json,hw06lib as L
d=L.load(sys.argv[1]); g=d["asset"]["representations"][sys.argv[2]]["geometry"]
g[sys.argv[4]]=[s for s in g[sys.argv[4]] if s["id"]!=sys.argv[3]]
json.dump(d,open(sys.argv[5],"w"))
PY
echo "without $v: $($B suit.ts "$G/t.json" | grep "^1:$SC ")"; done
for p in $(python3 -c "import hw06lib as L;print(' '.join(p['id'] for p in L.load('$SLUG')['asset']['poses']))"); do
python3 - "$SLUG" "$p" "$G/t.json" <<'PY'
import sys,json,hw06lib as L
d=L.load(sys.argv[1]); a=d["asset"]; a["poses"]=[p for p in a["poses"] if p["id"]==sys.argv[2]]; a["presentation"]["defaultPose"]=sys.argv[2]
json.dump(d,open(sys.argv[3],"w"))
PY
echo "only $p: $($B suit.ts "$G/t.json" | grep "^1:$SC ")"; done; rm -f "$G/t.json"
