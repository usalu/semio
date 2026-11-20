#!/bin/zsh
S=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R220
BUN=/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
python3 $S/mk.py >/dev/null
f="$S/new/$1"; cd "$(dirname "$f")" || exit 1
mkdir -p $S/art-new
PACKED_SOURCE="$S/new/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🟦️bun/🟨️.cjs" PACKED_ORIGIN="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🟦️bun/🟨️.cjs" SEMIO_TEST_ARTIFACT_DIR=$S/art-new BUN_RUNTIME_TRANSPILER_CACHE_PATH=0 R194_MAP=$S/map.json env -u SEMIO_FLEET_GROUP -u SEMIO_FLEET_NFS_HOST -u SEMIO_FLEET_NETWORK -u SEMIO_REMOTE_CACHE_ENDPOINT -u SEMIO_REMOTE_CACHE_ACCESS -u SEMIO_REMOTE_CACHE_INSTANCE -u SEMIO_REMOTE_CACHE_TOKEN nice -n 19 $BUN test --tsconfig-override $S/tsconfig.json --preload $S/preload.ts ./$(basename "$f") --timeout 180000 2>&1
