#!/bin/bash
# usage: ot.sh <stage-or-live test path relative to repo root or stage> ...  runs bun test with the R120 overlay (patched sources, staged tests)
cd /Users/ueli/Documents/semio || exit 1
D=$(dirname "$0"); case "$D" in /*) ;; *) D=$PWD/$D;; esac
BUN=$PWD/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
SC=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R120
for f in "$@"; do
 if [ -f "$SC/stage/$f" ]; then p="$SC/stage/$f"; else p="$PWD/$f"; fi
 echo "== $f"
 (cd "$(dirname "$p")" && env -u SEMIO_FLEET_GROUP -u SEMIO_FLEET_NFS_HOST -u SEMIO_FLEET_NETWORK -u SEMIO_REMOTE_CACHE_ENDPOINT -u SEMIO_REMOTE_CACHE_ACCESS -u SEMIO_REMOTE_CACHE_INSTANCE -u SEMIO_REMOTE_CACHE_TOKEN $BUN test --preload "$D/overlay.ts" "./$(basename "$p")" --timeout 180000 2>&1 | grep -E "^\(fail\)|^\(pass\)|error:|^ +[0-9]+ (pass|fail)|Expected|Received" | head -${LINES_MAX:-30})
done
