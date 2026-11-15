#!/bin/zsh
# usage: run-overlay.sh <test dir relative to the overlay> <file>
BUN=/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
HERE=${0:A:h}
OV=${P13_OVERLAY:-/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/P13/overlay}
export P13_OVERLAY=$OV P13_LIVE=/Users/Shared/semio
cd "$OV/$1" || exit 2
env -u SEMIO_FLEET_GROUP -u SEMIO_FLEET_NFS_HOST -u SEMIO_FLEET_NETWORK -u SEMIO_REMOTE_CACHE_ENDPOINT -u SEMIO_REMOTE_CACHE_ACCESS -u SEMIO_REMOTE_CACHE_INSTANCE -u SEMIO_REMOTE_CACHE_TOKEN BUN_RUNTIME_TRANSPILER_CACHE_PATH=0 $BUN test --preload $HERE/overlay-preload.ts ./$2 --timeout 120000 2>&1
