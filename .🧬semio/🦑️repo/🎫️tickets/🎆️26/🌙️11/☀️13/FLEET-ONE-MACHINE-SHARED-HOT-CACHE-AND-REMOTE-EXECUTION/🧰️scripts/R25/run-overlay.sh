#!/bin/zsh
# usage: run-overlay.sh <overlay dir> <test dir relative to the root> <file>  -- the test file lives in the overlay, edited files are served from it, everything else from the live tree
BUN=/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
HERE=${0:A:h}
export R5_OVERLAY=$1 R5_LIVE=${R5_LIVE:-/Users/Shared/semio} R5_PRELOAD=$HERE/overlay-preload.ts
cd "${R5_RUN_IN:-$R5_OVERLAY}/$2" || exit 2
env -u SEMIO_FLEET_GROUP -u SEMIO_FLEET_NFS_HOST -u SEMIO_FLEET_NETWORK -u SEMIO_REMOTE_CACHE_ENDPOINT -u SEMIO_REMOTE_CACHE_ACCESS -u SEMIO_REMOTE_CACHE_INSTANCE -u SEMIO_REMOTE_CACHE_TOKEN BUN_RUNTIME_TRANSPILER_CACHE_PATH=0 $BUN test --preload $HERE/overlay-preload.ts ./$3 --timeout 180000 2>&1
