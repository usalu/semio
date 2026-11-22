#!/bin/sh
# 🖼️ Renders the three-view sheet (light solid, third angle) of an HW-04 asset draft or catalog file at the given scales, optionally a pose or clip time, and rasterises them: render.sh <asset.json> <out-dir> [scales] [extra preview args]
set -e
H="$(cd "$(dirname "$0")" && pwd)"
B=/Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
S="/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage"
mkdir -p "$2"; F="$(cd "$(dirname "$1")" && pwd)/$(basename "$1")"; O="$(cd "$2" && pwd)"; SC="${3:-50 100 200 500}"; shift 3 2>/dev/null || shift $#
mkdir -p "$O"
ID=$(python3 -c "import json,sys;print(json.load(open(sys.argv[1]))['asset']['id'])" "$F")
for s in $SC; do
  (cd "$S" && $B 🔨️modules/🎨️depiction/🖨️preview/🟦️.ts --file "$F" --asset "$ID" --scale $s --views elevation,front,plan --projection third --view light-solid --out "$O" "$@" >/dev/null 2>&1 || true)
done
$B "$H/png.ts" 200 "$O"/*.svg >/dev/null
ls "$O"/*.png
