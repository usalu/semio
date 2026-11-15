#!/bin/bash
# Package T: runs one bun test file (argument 1 = tests directory, argument 2 = file, default 🟦️.ts) in a clean fleet environment.
cd "$1" || exit 2
export RUSTUP_HOME=${RUSTUP_HOME:-$HOME/.rustup}
exec env -u SEMIO_FLEET_GROUP -u SEMIO_FLEET_NFS_HOST -u SEMIO_FLEET_NETWORK -u SEMIO_REMOTE_CACHE_ENDPOINT -u SEMIO_REMOTE_CACHE_ACCESS -u SEMIO_REMOTE_CACHE_INSTANCE -u SEMIO_REMOTE_CACHE_TOKEN /Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun test "./${2:-🟦️.ts}" --timeout 240000
