#!/bin/zsh
# usage: t.sh <path relative to library root or absolute> [bun args]
L="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
f="$1"; shift; [[ "$f" = /* ]] || f="$L/$f"
BUN=/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
if ! grep -q "declareLibraryBuildTestEnvelopeV1(\|declareBuildTestEnvelopeV1(" "$f"; then export SEMIO_TEST_ARTIFACT_DIR=${SEMIO_TEST_ARTIFACT_DIR:-/private/tmp/e19a/t$$}; mkdir -p $SEMIO_TEST_ARTIFACT_DIR; fi
cd "$(dirname "$f")" && env -u SEMIO_FLEET_GROUP -u SEMIO_FLEET_NFS_HOST -u SEMIO_FLEET_NETWORK -u SEMIO_REMOTE_CACHE_ENDPOINT -u SEMIO_REMOTE_CACHE_ACCESS -u SEMIO_REMOTE_CACHE_INSTANCE -u SEMIO_REMOTE_CACHE_TOKEN BUN_RUNTIME_TRANSPILER_CACHE_PATH=0 SEMIO_TEST_PYTHON_BINARY=${SEMIO_TEST_PYTHON_BINARY:-/usr/bin/python3} $BUN test "./$(basename "$f")" --timeout 180000 "$@" 2>&1
