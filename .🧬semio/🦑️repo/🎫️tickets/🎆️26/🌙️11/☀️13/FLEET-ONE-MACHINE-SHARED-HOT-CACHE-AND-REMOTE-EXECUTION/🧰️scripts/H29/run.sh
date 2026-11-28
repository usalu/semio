#!/bin/zsh
# usage: run.sh <overlay-relative test file>
S=/private/tmp/claude-501/-Users-ueli--claude-spares-spare-fuDDx3/098f233d-9be4-4a1a-b4a8-48924f807f61/scratchpad/H29
BUN=/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
AJV=$(cd /Users/ueli/Documents/semio && $BUN -e 'console.log(Bun.resolveSync("ajv","/Users/ueli/Documents/semio"))')
F="$S/overlay/$1"; R="${F%.ts}.run.test.ts"
sed "s#from \"ajv\"#from \"$AJV\"#" "$F" > "$R"
cd ${RUNDIR:-$S} && env -u SEMIO_FLEET_GROUP -u SEMIO_FLEET_NFS_HOST -u SEMIO_FLEET_NETWORK -u SEMIO_REMOTE_CACHE_ENDPOINT -u SEMIO_REMOTE_CACHE_ACCESS -u SEMIO_REMOTE_CACHE_INSTANCE -u SEMIO_REMOTE_CACHE_TOKEN BUN_RUNTIME_TRANSPILER_CACHE_PATH=0 nice -n 10 $BUN test --preload $S/overlay-plugin.ts "$R" --timeout 120000 2>&1 | tail -${2:-70}
rm -f "$R"
