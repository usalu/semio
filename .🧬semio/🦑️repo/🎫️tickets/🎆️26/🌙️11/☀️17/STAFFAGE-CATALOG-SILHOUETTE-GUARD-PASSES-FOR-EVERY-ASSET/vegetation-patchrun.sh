#!/bin/bash
# usage: vegetation-patchrun.sh <slug> <scale>/<view> ...   patches the V-only clusters of the given views and refines the placement
HERE="$(cd "$(dirname "$0")" && pwd)"
G="$HERE/vegetation-work"
B=/Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
slug="$1"; shift
cd "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript"
$B "$HERE/vegetation-patch.ts" "$slug" "$G/patches-$slug.json" "$@" | tail -3
python3 "$HERE/vegetation-patch-apply.py" "$slug" "$G/patches-$slug.json" | tail -1
for i in 1 2 3 4; do
  $B "$HERE/vegetation-refine.ts" "$slug" "$G/edits-$slug.json" "$G/corr-$slug.json" | tail -1
  if [ "$(python3 -c "import json;print(len(json.load(open('$G/corr-$slug.json'))))")" = "0" ]; then break; fi
  python3 "$HERE/vegetation-apply.py" "$slug" --corrections "$G/corr-$slug.json" | tail -1
done
