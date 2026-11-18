#!/bin/zsh
# R183: run the new laws and the existing neighbours against the map in $S/map.json (see stage.sh). usage: tests.sh [files...]
S=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R183
BUN=/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
export BUN_RUNTIME_TRANSPILER_CACHE_PATH=0 R183_MAP=$S/map.json
SD=$(cd $(dirname $0) && pwd); M=$S/mirror/sidecar/🚦️ready
for f in "$@"; do d=$(dirname "$f"); b=$(basename "$f")
  out=$(cd "$d" && env -u SEMIO_FLEET_GROUP -u SEMIO_FLEET_NFS_HOST -u SEMIO_FLEET_NETWORK -u SEMIO_REMOTE_CACHE_ENDPOINT -u SEMIO_REMOTE_CACHE_ACCESS -u SEMIO_REMOTE_CACHE_INSTANCE -u SEMIO_REMOTE_CACHE_TOKEN nice -n 10 $BUN test --preload $SD/preload.ts ./$b --timeout 180000 2>&1)
  echo "== $f"; echo "$out" | /usr/bin/grep -E '^(\(pass\)|\(fail\)|error:| *[0-9]+ (pass|fail))|Expected|Received|error' | head -40
done
