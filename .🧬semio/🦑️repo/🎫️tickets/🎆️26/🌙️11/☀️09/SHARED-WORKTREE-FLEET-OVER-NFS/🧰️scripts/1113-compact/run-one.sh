#!/bin/bash
# usage: run-one.sh <module root> <relative test file> [bun test args]  -- one test file with the protocol's environment
W="$1"; F="$2"; shift 2
BUN=/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
cd "$W/$(dirname "$F")" && env -u SEMIO_FLEET_GROUP -u SEMIO_FLEET_NFS_HOST -u SEMIO_FLEET_NETWORK -u SEMIO_REMOTE_CACHE_ENDPOINT -u SEMIO_REMOTE_CACHE_ACCESS -u SEMIO_REMOTE_CACHE_INSTANCE "$BUN" test "./$(basename "$F")" --timeout 180000 "$@"
