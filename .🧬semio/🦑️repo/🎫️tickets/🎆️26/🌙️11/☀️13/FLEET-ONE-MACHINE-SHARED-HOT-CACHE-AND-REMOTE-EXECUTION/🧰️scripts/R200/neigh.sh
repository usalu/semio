#!/bin/zsh
MODE=$1
cd /Users/ueli/Documents/semio || exit 1
BUN=/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
S=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R200
B="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧭️script-inputs"
find "$B" -path '*🧪️tests*' \( -name '🟦️.ts' -o -name '*.test.ts' \) | sort | while read f; do
 d=$(dirname "$f"); n=$(basename "$f")
 if [ "$MODE" = live ]; then export NOSTAGE=1; else unset NOSTAGE; fi
 cd "$d"; out=$(BUN_RUNTIME_TRANSPILER_CACHE_PATH=0 env -u SEMIO_FLEET_GROUP -u SEMIO_FLEET_NFS_HOST -u SEMIO_FLEET_NETWORK -u SEMIO_REMOTE_CACHE_ENDPOINT -u SEMIO_REMOTE_CACHE_ACCESS -u SEMIO_REMOTE_CACHE_INSTANCE -u SEMIO_REMOTE_CACHE_TOKEN nice -n 10 $BUN test --preload $S/stage.ts ./$n --timeout 180000 2>&1)
 echo "$(echo "$out" | grep -a -E '^ *[0-9]+ (pass|fail)' | tr '\n' ' ') ${f#$B/}"
done
echo NEIGHDONE
