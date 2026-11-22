#!/bin/sh
# usage: render.sh <asset.json> <id> <scale> <pose> <view> <outdir> [density] [extra preview args]   draws the three views and the axonometric sheet of one pose and rasterises it
B=/Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
S="/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage"
here="$(cd "$(dirname "$0")" && pwd)"
file="$1"; id="$2"; scale="$3"; pose="$4"; view="$5"; out="$6"; density="${7:-200}"; shift 7 2>/dev/null
mkdir -p "$out"
(cd "$S" && "$B" "🔨️modules/🎨️depiction/🖨️preview/🟦️.ts" --asset "$id" --file "$file" --scale "$scale" --pose "$pose" --view "$view" --views elevation,front,plan,axonometric --projection third --no-png --flow none --out "$out" "$@" >/dev/null 2>&1)
"$B" "$here/png.ts" "$out" "$density"
