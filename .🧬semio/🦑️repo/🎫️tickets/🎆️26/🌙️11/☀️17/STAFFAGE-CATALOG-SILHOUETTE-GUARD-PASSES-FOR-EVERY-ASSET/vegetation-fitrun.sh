#!/bin/bash
# usage: vegetation-fitrun.sh <slug> <scale>/<view> ...   IoU profile fit of the solid-drawn plates, applied
HERE="$(cd "$(dirname "$0")" && pwd)"
G="$HERE/vegetation-work"
B=/Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
slug="$1"; shift
cd "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript"
for sv in "$@"; do
  scale="${sv%/*}"; view="${sv#*/}"
  $B "$HERE/vegetation-fit.ts" "$slug" "$scale" "$view" "$G/fit-$slug-$scale-$view.json" | tail -1
  python3 "$HERE/vegetation-fit-apply.py" "$slug" "$G/fit-$slug-$scale-$view.json" 0.002 | tail -1
done
