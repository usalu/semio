#!/bin/zsh
BUN=/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
mkdir -p /tmp/e19art 2>/dev/null
while read f; do
  [ -z "$f" ] && continue
  d=$(dirname "/Users/ueli/Documents/semio/$f"); b=$(basename "$f")
  export SEMIO_TEST_ARTIFACT_DIR=/private/tmp/e19a/l$$; mkdir -p $SEMIO_TEST_ARTIFACT_DIR
  out=$(cd "$d" && env -u SEMIO_FLEET_GROUP -u SEMIO_FLEET_NFS_HOST -u SEMIO_FLEET_NETWORK -u SEMIO_REMOTE_CACHE_ENDPOINT -u SEMIO_REMOTE_CACHE_ACCESS -u SEMIO_REMOTE_CACHE_INSTANCE -u SEMIO_REMOTE_CACHE_TOKEN BUN_RUNTIME_TRANSPILER_CACHE_PATH=0 SEMIO_TEST_PYTHON_BINARY=${SEMIO_TEST_PYTHON_BINARY:-/usr/bin/python3} $BUN test "./$b" --timeout 180000 2>&1)
  rc=$?
  echo "rc=$rc $(echo "$out" | grep -E '^\s*[0-9]+ (pass|fail)$' | tr -s ' \n' ' ') :: ${f#*library/}"
  [ $rc -ne 0 ] && echo "$out" | grep -E '^(error|\(fail\))' | head -4 | cut -c1-200
done
