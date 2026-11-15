#!/bin/zsh
# usage: run.sh <harness file name without dir> [args]; copies harness/<name> into the overlay test dir and runs it at the overlay location
HERE=${0:A:h}
OVL=${OVL:-overlay}
S=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R5c
LIB="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
OVT="$S/$OVL/$LIB/🏗️build/🕸️graph/📥️inference/🧪️tests/r5c"
mkdir -p "$OVT" && cp "$HERE/harness/$1" "$OVT/$1"
BUN=/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
export R5_OVERLAY=$S/$OVL R5_LIVE=/Users/Shared/semio BUN_RUNTIME_TRANSPILER_CACHE_PATH=0
cd "$OVT" && env -u SEMIO_FLEET_GROUP -u SEMIO_FLEET_NFS_HOST -u SEMIO_FLEET_NETWORK -u SEMIO_REMOTE_CACHE_ENDPOINT -u SEMIO_REMOTE_CACHE_ACCESS -u SEMIO_REMOTE_CACHE_INSTANCE -u SEMIO_REMOTE_CACHE_TOKEN $BUN --preload "$HERE/../R5/lab/overlay-preload.ts" "./$1" "${@:2}"
