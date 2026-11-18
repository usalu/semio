#!/bin/sh
# usage: humansb-A-trace.sh <asset> <scale> <view> <dilate|hull> <spec> <base.json> <out.json> [eps]
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17"
N="$T/STAFFAGE-CATALOG-SILHOUETTE-GUARD-PASSES-FOR-EVERY-ASSET"
B=/Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
cd "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript"
a=$1; s=$2; v=$3; dl=$4; spec=$5; base=$6; out=$7; eps=${8:-0.25}
tmp="$N/🗑️generated/humansb-A-trace-tmp.json"
if [ "$dl" = "hull" ]; then
  EPS=$eps PATCH="$base" $B "$N/humansb-A-hullfit.ts" $a $s $v "$tmp" "$spec" > /dev/null
else
  DILATE=$dl EPS=$eps TRACE=1 PATCH="$base" $B "$N/humansb-A-hullfit.ts" $a $s $v "$tmp" "$spec" > /dev/null
fi
python3 "$N/humansb-A-merge.py" "$out" "$base" "$tmp"
rm -f "$tmp"
