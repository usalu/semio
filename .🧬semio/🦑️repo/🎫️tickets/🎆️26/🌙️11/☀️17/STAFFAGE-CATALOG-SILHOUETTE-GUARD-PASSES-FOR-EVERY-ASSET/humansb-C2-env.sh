#!/bin/bash
# 🧭 Environment of the cyclist pass: source it, then use `tr <patches> [scales]`, `mask <scale> <view> x0 x1 y0 y1 step <patches>`.
N="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-CATALOG-SILHOUETTE-GUARD-PASSES-FOR-EVERY-ASSET"
G="$N/🗑️generated"
B=/Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
TS="/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript"
tr() { local p=$1; shift; (cd "$TS" && $B "$N/humansb-C-try.ts" cyclist "$p" "$@"); }
mask() { (cd "$TS" && $B "$N/humansb-C-mask.ts" cyclist "$@"); }
sw() { (cd "$TS" && $B "$N/humansb-C-sw.ts" cyclist "$@"); }
locp() { (cd "$TS" && $B "$N/humansb-C2-locp.ts" cyclist "$@"); }
