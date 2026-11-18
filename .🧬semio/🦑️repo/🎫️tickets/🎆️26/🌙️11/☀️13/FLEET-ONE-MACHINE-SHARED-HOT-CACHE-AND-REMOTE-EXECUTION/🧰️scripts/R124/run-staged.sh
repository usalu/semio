#!/bin/zsh
# usage: run-staged.sh <test file relative to staged tests dir> [--timeout ms]
S=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R124
R=/Users/ueli/Documents/semio
O="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🌐️owner-pool"
SC="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🛰️sidecar/🏊️pool"
[ -z "$R124_BASE" ] && export R124_OVER="$O/🟦️.ts=$S/stage/owner-pool.ts
$O/🏃️runner/🟨️.mjs=$S/stage/runner.mjs
$O/🧬️schema/🔣️.json=$S/stage/schema.json
$O/🧮️lifecycle/🟦️.ts=$S/stage/lifecycle.ts
$SC/🟦️.ts=$S/stage/sidecar-pool.ts"
cd "$S/tests" || exit 1
env -u SEMIO_FLEET_GROUP -u SEMIO_FLEET_NFS_HOST -u SEMIO_FLEET_NETWORK -u SEMIO_REMOTE_CACHE_ENDPOINT -u SEMIO_REMOTE_CACHE_ACCESS -u SEMIO_REMOTE_CACHE_INSTANCE -u SEMIO_REMOTE_CACHE_TOKEN $R/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun test --preload "$R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/🧰️scripts/R124/overlay.ts" "./$1" --timeout ${2:-180000}
