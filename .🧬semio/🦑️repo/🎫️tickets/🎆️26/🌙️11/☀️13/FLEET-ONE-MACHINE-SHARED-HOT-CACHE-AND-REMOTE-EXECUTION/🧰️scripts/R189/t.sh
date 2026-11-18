#!/bin/zsh
S=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R189
BUN=/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
export BUN_RUNTIME_TRANSPILER_CACHE_PATH=0 R189_MAP=$S/map.json TMPDIR=$S/tmp/
for f in "$@"; do
 st=$(python3 -c "import json,sys;m=json.load(open('$S/map.json'));print(m.get(sys.argv[1],sys.argv[1]))" "$f"); d=$(dirname "$st"); b=$(basename "$st")
 out=$(cd "$d" && env -u SEMIO_FLEET_GROUP -u SEMIO_FLEET_NFS_HOST -u SEMIO_FLEET_NETWORK -u SEMIO_REMOTE_CACHE_ENDPOINT -u SEMIO_REMOTE_CACHE_ACCESS -u SEMIO_REMOTE_CACHE_INSTANCE -u SEMIO_REMOTE_CACHE_TOKEN nice -n 10 $BUN test --preload $S/preload.ts ./$b --timeout 180000 2>&1)
 echo "== $(basename $(dirname $f)) $(echo "$out"|/usr/bin/grep -E '^ *[0-9]+ (pass|fail)'|tr -d ' '|tr '\n' ' ')"; echo "$out"|/usr/bin/grep -E '^\(fail\)|^error|Expected|Received|^ *at .*(🟦️)|rror:' | head -${LINES_MAX:-14}
done
