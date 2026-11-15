#!/bin/zsh
B=/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
S=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/I
mkdir -p $S/art
for f in "$@"; do
  d=$(dirname "$f"); n=$(basename "$f")
  out=$(cd "$d" && env -u SEMIO_FLEET_GROUP -u SEMIO_FLEET_NFS_HOST -u SEMIO_FLEET_NETWORK -u SEMIO_REMOTE_CACHE_ENDPOINT -u SEMIO_REMOTE_CACHE_ACCESS -u SEMIO_REMOTE_CACHE_INSTANCE -u SEMIO_REMOTE_CACHE_TOKEN SEMIO_TEST_ARTIFACT_DIR=$S/art $B test --tsconfig-override $S/tsconfig.json "./$n" --timeout 180000 2>&1)
  echo "$f :: $(echo "$out" | grep -E '^ [0-9]+ (pass|fail)' | tr '\n' ' ') $(echo "$out" | grep -E '^error:' | head -2 | tr '\n' '|' | cut -c1-200)"
  echo "$out" | grep -E '^\(fail\)' | head -8 | cut -c1-200
done
