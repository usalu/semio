#!/bin/bash
B=/Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
OLD="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-WEBSITE-CDN-REDEPLOY-READINESS"
NEW="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-CATALOG-SILHOUETTE-GUARD-PASSES-FOR-EVERY-ASSET"
cd "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript"
tool="$1"; shift
case "$tool" in
  triage) out="$1"; shift; $B "$OLD/silhouette-triage.ts" current "$NEW/🗑️generated/$out" "$@";;
  evalall) $B "$NEW/vehicles-evalall.ts" "$@";;
  vdiag) $B "$NEW/vehicles-diag.ts" "$@";;
  owner) $B "$NEW/vehicles-owner.ts" "$@";;
  scan) $B "$NEW/vehicles-scan.ts" "$@";;
  who) $B "$NEW/vehicles-who.ts" "$@";;
  outline) $B "$NEW/vehicles-outline.ts" "$@";;
  extj) $B "$NEW/vehicles-ext.ts" "$@";;
  tiles) $B "$NEW/vehicles-tiles.ts" "$@";;
  bones) $B "$NEW/vehicles-bones.ts" "$@";;
  diag) $B "$OLD/silhouette-diag.ts" "$@";;
  ext) $B "$OLD/solids-extent.ts" "$@";;
  world) $B "$OLD/shapes-world.ts" "$@";;
  bins) $B "$OLD/iou-bins.ts" "$@";;
  sw) $B "$OLD/stroke-widths.ts" "$@";;
esac
