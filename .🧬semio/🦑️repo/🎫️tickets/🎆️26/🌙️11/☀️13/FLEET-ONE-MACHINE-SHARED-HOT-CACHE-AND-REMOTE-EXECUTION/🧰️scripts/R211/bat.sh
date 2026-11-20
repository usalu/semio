#!/bin/zsh
# usage: bat.sh <mode> <origdir>...   prints pass/fail per dir
S=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R211
BUN=/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
mode=$1; shift
python3 $S/mk.py $mode
for d in "$@"; do
 n=$(echo "$d"|md5 -q|cut -c1-8); out=$S/rb-$mode/$n; rm -rf $out
 python3 $S/gen.py "$d" "$out"
 cd $out
 res=$(BUN_RUNTIME_TRANSPILER_CACHE_PATH=0 R194_MAP=$S/map.$mode.json R211_PRELOAD=$S/preload.ts env -u SEMIO_FLEET_GROUP -u SEMIO_FLEET_NFS_HOST -u SEMIO_FLEET_NETWORK -u SEMIO_REMOTE_CACHE_ENDPOINT -u SEMIO_REMOTE_CACHE_ACCESS -u SEMIO_REMOTE_CACHE_INSTANCE -u SEMIO_REMOTE_CACHE_TOKEN nice -n 10 $BUN test --preload $S/preload.ts ./🟦️.ts --timeout 180000 2>&1 | grep -E "^ *[0-9]+ (pass|fail)"|tr '\n' ' ')
 echo "$mode $(basename $(dirname $d))/$(basename $d): $res"
done
