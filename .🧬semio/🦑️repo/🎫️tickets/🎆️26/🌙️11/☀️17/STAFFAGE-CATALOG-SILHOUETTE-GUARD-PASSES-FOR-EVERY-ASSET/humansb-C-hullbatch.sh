#!/bin/bash
# 🔶 Batch of hullshape ops. usage: humansb-C-hullbatch.sh <asset> <scale> <view> <bone> <out.json> <patches|-> <mode> "<ref newId solidIds>" ...
B=/Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
D="$(cd "$(dirname "$0")" && pwd)"
cd "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript" || exit 1
asset=$1 scale=$2 view=$3 bone=$4 out=$5 patch=$6 mode=$7; shift 7
for spec in "$@"; do
  read -r ref id ids role z <<<"$spec"
  $B "$D/humansb-C-hullshape.ts" "$asset" "$scale" "$view" "$bone" "$mode" "$ref" "$id" "$ids" "${role:-body}" "${z:-5}" "${patch:--}"
done | python3 -c "
import sys, json
json.dump([json.loads(l) for l in sys.stdin if l.strip()], open('$out', 'w'))"
python3 -c "import json; print(len(json.load(open('$out'))), 'ops')"
