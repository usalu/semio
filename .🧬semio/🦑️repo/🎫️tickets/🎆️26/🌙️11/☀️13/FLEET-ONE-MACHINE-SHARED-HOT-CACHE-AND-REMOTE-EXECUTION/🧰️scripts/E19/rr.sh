#!/bin/zsh
# raw run from the checkout root, as the suite runner does (cwd = checkout, root bunfig.toml preloads): usage rr.sh <path under library or absolute>
L="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
f="$1"; shift; [[ "$f" = /* ]] || f="$L/$f"
BUN=/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
cd /Users/ueli/Documents/semio && env -u SEMIO_FLEET_GROUP -u SEMIO_FLEET_NFS_HOST -u SEMIO_FLEET_NETWORK -u SEMIO_REMOTE_CACHE_ENDPOINT -u SEMIO_REMOTE_CACHE_ACCESS -u SEMIO_REMOTE_CACHE_INSTANCE -u SEMIO_REMOTE_CACHE_TOKEN -u SEMIO_TEST_ARTIFACT_DIR BUN_RUNTIME_TRANSPILER_CACHE_PATH=0 SEMIO_TEST_PYTHON_BINARY=${SEMIO_TEST_PYTHON_BINARY:-/usr/bin/python3} $BUN test "./${f#/Users/ueli/Documents/semio/}" --timeout 180000 "$@" 2>&1
