#!/bin/zsh
# usage: suite.sh <overlay> <test dir relative to root>...  : prints live baseline and overlay counts
S=${0:A:h}; ov=$1; shift
for d in "$@"; do
  for f in $(ls "/Users/Shared/semio/$d" | grep -E 'ts$'); do
    [[ "$f" == "🟦️.ts" ]] || continue
    live=$(cd "/Users/Shared/semio/$d" && env -u SEMIO_FLEET_GROUP BUN_RUNTIME_TRANSPILER_CACHE_PATH=0 SEMIO_TEST_ARTIFACT_DIR=$S/sink /Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun test ./$f --timeout 240000 2>&1 | grep -E '^ *[0-9]+ (pass|fail)$' | tr '\n' ' ')
    mkdir -p "$S/$ov/$d"; cp -Rn "/Users/Shared/semio/$d/" "$S/$ov/$d/" 2>/dev/null
    python3 $S/fixspec.py "$S/$ov/$d/$f" > /dev/null
    over=$($S/rig.sh $ov "$d" $f 2>&1 | grep -E '^ *[0-9]+ (pass|fail)$' | tr '\n' ' ')
    echo "${d##*/} | live: $live | overlay: $over"
  done
done
