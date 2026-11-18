#!/bin/zsh
S=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R129
R=/Users/ueli/Documents/semio
PH="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
FARM="$PH/🕸️graph/📥️inference/📦️project-nodes/🏊️project-farm"
export R129_OVER="$FARM/🏊️pool/🟨️.mjs=$S/stage/pool/pool.staged.mjs"
[ -n "$R129_F2" ] && R129_OVER="$FARM/🏊️pool/🟨️.mjs=$S/stage/f2/pool.mjs
$FARM/🏊️pool/🧬️schema/🔣️.json=$S/stage/f2/schema.json
$FARM/👷️service/🟨️.mjs=$S/stage/f2/service.mjs
$FARM/👷️worker/🟨️.mjs=$S/stage/f2/worker.mjs"
[ -n "$R129_BASE" ] && unset R129_OVER
cd "$S/tests" || exit 1
env -u SEMIO_FLEET_GROUP -u SEMIO_FLEET_NFS_HOST -u SEMIO_FLEET_NETWORK -u SEMIO_REMOTE_CACHE_ENDPOINT -u SEMIO_REMOTE_CACHE_ACCESS -u SEMIO_REMOTE_CACHE_INSTANCE -u SEMIO_REMOTE_CACHE_TOKEN $R/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun test --preload "$R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/🧰️scripts/R129/overlay.ts" "$@" --timeout 180000
