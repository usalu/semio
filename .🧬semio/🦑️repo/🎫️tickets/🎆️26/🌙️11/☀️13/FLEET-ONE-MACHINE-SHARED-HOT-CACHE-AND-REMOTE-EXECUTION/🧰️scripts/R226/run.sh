#!/bin/zsh
# usage: run.sh <test dir> [staged|base]; staged also preloads the staged files into child bun processes (BUN_OPTIONS)
S=/private/tmp/claude-501/-Users-ueli-Documents-semio/60403654-b0e9-4c0d-a7d3-fba5340a3de9/scratchpad/R226
BUN=/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
dir="$1"; mode=${2:-staged}
cd "$dir" || exit 9
pre=(); opt=""; [ "$mode" = staged ] && { pre=(--preload $S/preload.ts); opt="--preload $S/preload.ts"; }
env -u SEMIO_FLEET_GROUP -u SEMIO_FLEET_NFS_HOST -u SEMIO_FLEET_NETWORK -u SEMIO_REMOTE_CACHE_ENDPOINT -u SEMIO_REMOTE_CACHE_ACCESS -u SEMIO_REMOTE_CACHE_INSTANCE -u SEMIO_REMOTE_CACHE_TOKEN BUN_RUNTIME_TRANSPILER_CACHE_PATH=0 R226_MAP=$S/map.tsv nice -n 19 $BUN test $pre ./🟦️.ts --timeout 180000 2>&1
