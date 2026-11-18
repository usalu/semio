#!/bin/zsh
# usage: runtest.sh <map.json> <outfile> <testfile>
S=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R193
B=/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
f="$3"; cd "$(dirname "$f")" || exit 1
R193_MAP=$1 BUN_RUNTIME_TRANSPILER_CACHE_PATH=0 env -u BUN_OPTIONS -u SEMIO_FLEET_GROUP -u SEMIO_FLEET_NFS_HOST -u SEMIO_FLEET_NETWORK -u SEMIO_REMOTE_CACHE_ENDPOINT -u SEMIO_REMOTE_CACHE_ACCESS -u SEMIO_REMOTE_CACHE_INSTANCE -u SEMIO_REMOTE_CACHE_TOKEN nice -n 10 $B test --preload $S/preload.ts ./"$(basename $f)" --timeout 600000 > $2 2>&1
grep -E "^\((pass|fail)\)|^ *[0-9]+ (pass|fail)|^error:|Expected|Received" $2 | head -${LINES_MAX:-40}
