#!/bin/sh
# usage: humansb-A-d.sh <asset> <scale> <patch.json|-> [view [x y r]]
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17"
B=/Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
cd "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript"
a=$1; s=$2; p=$3; shift 3
if [ "$p" = "-" ]; then unset PATCH; else export PATCH="$p"; fi
exec $B "$T/STAFFAGE-CATALOG-SILHOUETTE-GUARD-PASSES-FOR-EVERY-ASSET/humansb-A-diag.ts" $a $s "$@"
