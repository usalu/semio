#!/bin/zsh
S=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R181
export SEMIO_TEST_ARTIFACT_DIR=$S/artifacts R181_STAGED=$S BUN_RUNTIME_TRANSPILER_CACHE_PATH=0
BUN=/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
cd "$1" || exit 2
P=(); [ -n "$PRELOAD" ] && { P=(--preload $S/preload.ts); export BUN_OPTIONS=--preload=$S/preload.ts; }
env -u SEMIO_FLEET_GROUP -u SEMIO_FLEET_NFS_HOST -u SEMIO_FLEET_NETWORK -u SEMIO_REMOTE_CACHE_ENDPOINT -u SEMIO_REMOTE_CACHE_ACCESS -u SEMIO_REMOTE_CACHE_INSTANCE -u SEMIO_REMOTE_CACHE_TOKEN nice -n 10 $BUN test "${P[@]}" "$2" --timeout 180000 2>&1 | tail -${TAIL:-60} | cut -c1-600
