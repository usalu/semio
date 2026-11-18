#!/bin/bash
# usage: vegetation-hsearch-run.sh <slug> <iterations> <scale>/<view> ...   searches, then applies the profile scales of every view
HERE="$(cd "$(dirname "$0")" && pwd)"
G="$HERE/vegetation-work"
B=/Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
slug="$1"; iters="$2"; shift 2
cd "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript"
for sv in "$@"; do
  scale="${sv%/*}"; view="${sv#*/}"
  $B "$HERE/vegetation-hsearch.ts" "$slug" "$scale" "$view" "$G/hs-$slug-$scale-$view.json" "$iters" 2>&1 | grep "hsearch\|rror"
  python3 "$HERE/vegetation-fit-apply.py" "$slug" "$G/hs-$slug-$scale-$view.json" 0 | tail -1
done
