#!/bin/sh
# usage: run-files.sh <library-relative file>... ; runs each bun test file in its own directory without the fleet environment, prints one summary line each
L="/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
BUN=/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
for f in "$@"; do
  d=$(dirname "$L/$f"); b=$(basename "$f")
  out=$(cd "$d" && env -u SEMIO_FLEET_GROUP -u SEMIO_FLEET_NFS_HOST -u SEMIO_FLEET_NETWORK -u SEMIO_REMOTE_CACHE_ENDPOINT -u SEMIO_REMOTE_CACHE_ACCESS -u SEMIO_REMOTE_CACHE_INSTANCE -u SEMIO_REMOTE_CACHE_TOKEN "$BUN" test "./$b" --timeout 180000 2>&1)
  echo "$f :: $(echo "$out" | grep -E '^ *[0-9]+ (pass|fail)' | tr -s ' \n' ' ')"
  echo "$out" | grep -E '^\(fail\)|error:' | head -5
done
