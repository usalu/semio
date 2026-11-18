#!/bin/zsh
# usage: run.sh <overlay|live> <path relative to /Users/Shared/semio of test dir>
HERE=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R42
BUN=/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
if [ "$1" = overlay ]; then ROOT=$HERE/ov; EXTRA=(--preload $HERE/overlay-preload.ts); export R5_OVERLAY=$HERE/ov R5_LIVE=/Users/Shared/semio R5_PRELOAD=$HERE/overlay-preload.ts; else ROOT=/Users/Shared/semio; EXTRA=(); fi
if [ "$1" = overlay ] && [ -e "/Users/Shared/semio/$2/🟦️.ts" ]; then ROOT=/Users/Shared/semio; fi
cd "$ROOT/$2" || exit 1
env -u SEMIO_FLEET_GROUP -u SEMIO_FLEET_NFS_HOST -u SEMIO_FLEET_NETWORK -u SEMIO_REMOTE_CACHE_ENDPOINT -u SEMIO_REMOTE_CACHE_ACCESS -u SEMIO_REMOTE_CACHE_INSTANCE -u SEMIO_REMOTE_CACHE_TOKEN BUN_RUNTIME_TRANSPILER_CACHE_PATH=0 $BUN test $EXTRA ./🟦️.ts --timeout 180000 2>&1 | tail -${3:-60}
