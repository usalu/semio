#!/bin/zsh
S=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/P2
BUN=/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
f="/Users/Shared/semio/$1"; shift
cd "$(dirname "$f")" && env -u SEMIO_FLEET_GROUP -u SEMIO_FLEET_NFS_HOST -u SEMIO_FLEET_NETWORK -u SEMIO_REMOTE_CACHE_ENDPOINT -u SEMIO_REMOTE_CACHE_ACCESS -u SEMIO_REMOTE_CACHE_INSTANCE -u SEMIO_REMOTE_CACHE_TOKEN SEMIO_TEST_ARTIFACT_DIR=$S/artifacts P2_LIVE=/Users/Shared/semio P2_MIRROR=$S/mirror $BUN test --preload $S/tools/overlay.ts ./"$(basename "$f")" --timeout 180000 "$@"
