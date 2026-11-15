#!/bin/zsh
# usage: runtest.sh <new test source in this dir> <test dir relative to LIB> [extra bun args]; runs the test from the overlay at its overlay location
HERE=${0:A:h}
OVL=${OVL:-overlay}
S=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R5c
LIB="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
DEST="$S/$OVL/$LIB/$2"
mkdir -p "$DEST" && cp "$HERE/$1" "$DEST/🟦️.ts"
SCH="$LIB/🏃️process/🔌️providers/🧬️schema"; mkdir -p "$S/$OVL/$SCH" && cp "/Users/Shared/semio/$SCH/🔣️.json" "$S/$OVL/$SCH/🔣️.json"
BUN=/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
export R5_OVERLAY=$S/$OVL R5_LIVE=/Users/Shared/semio BUN_RUNTIME_TRANSPILER_CACHE_PATH=0
cd "$DEST" && env -u SEMIO_FLEET_GROUP -u SEMIO_FLEET_NFS_HOST -u SEMIO_FLEET_NETWORK -u SEMIO_REMOTE_CACHE_ENDPOINT -u SEMIO_REMOTE_CACHE_ACCESS -u SEMIO_REMOTE_CACHE_INSTANCE -u SEMIO_REMOTE_CACHE_TOKEN $BUN test --preload "$HERE/../R5/lab/overlay-preload.ts" ./🟦️.ts --timeout 240000 "${@:3}" 2>&1
