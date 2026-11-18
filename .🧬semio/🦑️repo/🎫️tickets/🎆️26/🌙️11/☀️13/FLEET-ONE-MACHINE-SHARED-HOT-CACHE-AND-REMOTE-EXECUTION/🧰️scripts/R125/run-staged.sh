#!/bin/zsh
S=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R125
R=/Users/ueli/Documents/semio
HC="🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity"
PH="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
export R125_OVER="$HC/📏️decision/🟦️.ts=$S/stage/decision.ts
$HC/🎫️gate/🟦️.ts=$S/stage/gate.ts
$HC/🪜️phases/🟦️.ts=$S/stage/phases.ts
$HC/🧬️schema/🔣️.json=$S/stage/schema.json
$PH/🔣️policy.json=$S/stage/policy.json
$PH/🕸️graph/📥️inference/📦️project-nodes/🏊️project-farm/🟨️.mjs=$S/stage/farm.mjs
$HC/🧪️tests/🟦️.ts=$S/stage/t-main.ts
$PH/🌡️host-capacity/🧪️tests/📉️shrink/🟦️.ts=$S/stage/t-shrink.ts
$PH/🌡️host-capacity/🧪️tests/🟦️.ts=$S/stage/t-product.ts"
[ -n "$R125_BASE" ] && unset R125_OVER
cd "$S/tests" || exit 1
env -u SEMIO_FLEET_GROUP -u SEMIO_FLEET_NFS_HOST -u SEMIO_FLEET_NETWORK -u SEMIO_REMOTE_CACHE_ENDPOINT -u SEMIO_REMOTE_CACHE_ACCESS -u SEMIO_REMOTE_CACHE_INSTANCE -u SEMIO_REMOTE_CACHE_TOKEN $R/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun test --preload "$R/$(dirname "$0" | sed "s|^$R/||")/overlay.ts" "$@" --timeout 180000
