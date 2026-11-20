#!/bin/zsh
S=/private/tmp/claude-501/-Users-ueli-Documents-semio/60403654-b0e9-4c0d-a7d3-fba5340a3de9/scratchpad/R229
export R229_MIRROR=$S/mirror R229_LIVE=/Users/ueli/Documents/semio BUN_RUNTIME_TRANSPILER_CACHE_PATH=0
BUN=/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
sed 's/R206_/R229_/g;s/"r206"/"r229"/' $S/../R229/preload.ts > $S/preload229.ts
RD="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🛰️sidecar/🚦️ready"
cd "$S/mirror/$RD/🧪️tests/$1" && env -u SEMIO_FLEET_GROUP -u SEMIO_FLEET_NFS_HOST -u SEMIO_FLEET_NETWORK -u SEMIO_REMOTE_CACHE_ENDPOINT -u SEMIO_REMOTE_CACHE_ACCESS -u SEMIO_REMOTE_CACHE_INSTANCE -u SEMIO_REMOTE_CACHE_TOKEN nice -n 19 $BUN test --preload $S/preload229.ts ./🟦️.ts --timeout 180000 2>&1 | tail -40
