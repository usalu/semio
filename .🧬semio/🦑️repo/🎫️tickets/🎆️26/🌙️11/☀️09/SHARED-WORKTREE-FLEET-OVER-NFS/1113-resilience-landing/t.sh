#!/bin/bash
ROOT=$1; F=$2
M="$ROOT/🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree"
BUN=/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
D=$(dirname "$M/$F"); B=$(basename "$F")
cd "$D" && env -u SEMIO_FLEET_GROUP -u SEMIO_FLEET_NFS_HOST -u SEMIO_FLEET_NETWORK -u SEMIO_REMOTE_CACHE_ENDPOINT -u SEMIO_REMOTE_CACHE_ACCESS -u SEMIO_REMOTE_CACHE_INSTANCE $BUN test "./$B" --timeout 60000 2>&1 | tail -${3:-25}
