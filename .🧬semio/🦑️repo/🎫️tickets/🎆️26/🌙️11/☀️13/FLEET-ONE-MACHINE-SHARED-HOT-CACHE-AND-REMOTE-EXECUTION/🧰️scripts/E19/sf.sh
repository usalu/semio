#!/bin/zsh
# faithful single-file run: the suite runner's own pipeline (row, envelope, native caller grant, launchers, sandbox) without the graph read
L="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
f="$1"; [[ "$f" = /* ]] || f="$L/$f"
BUN=/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
cd /Users/ueli/Documents/semio && env -u SEMIO_FLEET_GROUP -u SEMIO_FLEET_NFS_HOST -u SEMIO_FLEET_NETWORK -u SEMIO_REMOTE_CACHE_ENDPOINT -u SEMIO_REMOTE_CACHE_ACCESS -u SEMIO_REMOTE_CACHE_INSTANCE -u SEMIO_REMOTE_CACHE_TOKEN -u SEMIO_TEST_ARTIFACT_DIR BUN_RUNTIME_TRANSPILER_CACHE_PATH=0 SEMIO_TEST_PYTHON_BINARY=${SEMIO_TEST_PYTHON_BINARY:-/usr/bin/python3} $BUN run "$(dirname "$0")/suite-file.ts" "$f" 2>&1 | grep -v "^\[script\]"
exit ${pipestatus[1]}
