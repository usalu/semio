#!/bin/sh
# 🖼️ Renders an HW-13 asset: the default pose as a three-view sheet at 1:50, 1:100, 1:200 and 1:500 (light solid), every pose at 1:50 and the
# thermal and structural views at 1:100, then rasterises with sharp. Usage: render.sh <slug> [pose ...]
set -e
S="/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage"
B=/Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
H="$(cd "$(dirname "$0")" && pwd)"
O="$H/../../🗑️generated/HW-13/png/$1"
slug=$1; shift
mkdir -p "$O"
cd "$S"
P="🔨️modules/🎨️depiction/🖨️preview/🟦️.ts"
for sc in 50 100 200 500; do
  $B "$P" --asset "$slug" --scale $sc --view light-solid --views elevation,front,plan --projection third --flow none --no-png --out "$O/s$sc" >/dev/null 2>&1 || echo "[DEBUG] fail $sc"
done
for pose in "$@"; do
  $B "$P" --asset "$slug" --scale 50 --pose "$pose" --view light-solid --views elevation,front,plan --projection third --flow none --no-png --out "$O/poses" >/dev/null 2>&1 || echo "[DEBUG] fail $pose"
done
for v in thermal forces; do
  $B "$P" --asset "$slug" --scale 100 --view $v --viewpoint elevation --flow none --no-png --out "$O/$v" >/dev/null 2>&1 || echo "[DEBUG] fail $v"
done
$B "$H/png.ts" "$O" 150 >/dev/null
find "$O" -name "*.png" | sort
