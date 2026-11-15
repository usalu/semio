#!/bin/zsh
# usage: run-overlay.sh <overlay dir> <test file relative to the overlay>
BUN=/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
HERE=${0:A:h}
export R2b_OVERLAY=$1 R2b_LIVE=${R2b_LIVE:-/Users/Shared/semio}
cd "$1/${2:h}" || exit 2
env -u SEMIO_FLEET_GROUP -u SEMIO_FLEET_NFS_HOST -u SEMIO_FLEET_NETWORK -u SEMIO_REMOTE_CACHE_ENDPOINT -u SEMIO_REMOTE_CACHE_ACCESS -u SEMIO_REMOTE_CACHE_INSTANCE -u SEMIO_REMOTE_CACHE_TOKEN BUN_RUNTIME_TRANSPILER_CACHE_PATH=0 $BUN test --preload $HERE/overlay-preload.ts ./${2:t} --timeout 120000 2>&1
