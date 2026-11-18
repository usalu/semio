#!/bin/bash
# 🏃 Runs a ticket tool of the redeploy ticket with bun from the typescript package. humansb-B-run.sh <tool.ts> args...
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-WEBSITE-CDN-REDEPLOY-READINESS"
B=/Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
cd "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript"
tool="$1"; shift
HERE="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-CATALOG-SILHOUETTE-GUARD-PASSES-FOR-EVERY-ASSET"
case "$tool" in /*) ;; humansb-*) tool="$HERE/$tool";; *) tool="$T/$tool";; esac
exec $B "$tool" "$@"
