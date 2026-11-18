#!/bin/zsh
cd /Users/ueli/Documents/semio || exit 1
BUN=/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
S=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R200
echo "== RED (live code)"
NOSTAGE=1 BUN_RUNTIME_TRANSPILER_CACHE_PATH=0 env -u SEMIO_FLEET_GROUP -u SEMIO_FLEET_NFS_HOST -u SEMIO_FLEET_NETWORK -u SEMIO_REMOTE_CACHE_ENDPOINT -u SEMIO_REMOTE_CACHE_ACCESS -u SEMIO_REMOTE_CACHE_INSTANCE -u SEMIO_REMOTE_CACHE_TOKEN SEMIO_WORKSPACE_ROOT=$PWD $BUN test --preload $S/stage.ts $S/law/staged-law.test.ts --timeout 900000 2>&1 | grep -a "pass\|fail\|expect\|Expected\|Received\|error" | head -12 | cut -c1-300
echo "== GREEN (staged code)"
BUN_RUNTIME_TRANSPILER_CACHE_PATH=0 env -u SEMIO_FLEET_GROUP -u SEMIO_FLEET_NFS_HOST -u SEMIO_FLEET_NETWORK -u SEMIO_REMOTE_CACHE_ENDPOINT -u SEMIO_REMOTE_CACHE_ACCESS -u SEMIO_REMOTE_CACHE_INSTANCE -u SEMIO_REMOTE_CACHE_TOKEN SEMIO_WORKSPACE_ROOT=$PWD $BUN test --preload $S/stage.ts $S/law/staged-law.test.ts --timeout 900000 2>&1 | grep -a "pass\|fail\|expect\|Expected\|Received\|error" | head -12 | cut -c1-300
echo LAWDONE
