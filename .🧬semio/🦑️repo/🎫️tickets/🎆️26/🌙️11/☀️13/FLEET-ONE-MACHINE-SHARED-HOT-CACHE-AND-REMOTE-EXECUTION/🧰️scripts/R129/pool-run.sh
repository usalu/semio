#!/bin/bash
# usage: pool-run.sh <label> <owners-csv> [staged]   env R129_MB
cd /Users/ueli/Documents/semio || exit 1
BUN=$PWD/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
D="$PWD/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/🧰️scripts/R129"
SP=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R129
FARM="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🏊️project-farm"
if [ "$3" = staged ]; then export R129_PRE="$D/overlay.ts"; export R129_OVER="$FARM/🏊️pool/🟨️.mjs=$SP/stage/f2/pool.mjs
$FARM/🏊️pool/🧬️schema/🔣️.json=$SP/stage/f2/schema.json
$FARM/👷️service/🟨️.mjs=$SP/stage/f2/service.mjs
$FARM/👷️worker/🟨️.mjs=$SP/stage/f2/worker.mjs"; PRE=(--preload "$D/overlay.ts"); else PRE=(); fi
env -u SEMIO_FLEET_GROUP R129_OWNERS="$2" nice -n 5 /usr/bin/time -l $BUN "${PRE[@]}" "$D/pool-run.ts" > $SP/$1.out 2>&1
echo done >> $SP/$1.out
