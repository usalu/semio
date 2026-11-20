#!/bin/zsh
S=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R220
BUN=/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
P="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies"
python3 $S/mk.py >/dev/null
mkdir -p $S/art-bat
for mode in base new; do
 for d in "💾️storage/🧪️tests/👓️reading" "💾️storage/🧪️tests/🏭️producer" "💾️storage/🧪️tests/🌱️seeded" "💾️storage/🧪️tests" "🧾️incremental/🧪️tests" "🧾️generation/🧪️tests/🛡️followup" "🧾️generation/🧪️tests/🚪️ingress" "🧾️generation/🧪️tests/🌡️heap-environment"; do
  cd "$P/$d" || continue
  if [ $mode = new ]; then extra=(--tsconfig-override $S/tsconfig.json --preload $S/preload.ts); else extra=(); fi
  out=$(SEMIO_TEST_ARTIFACT_DIR=$S/art-bat BUN_RUNTIME_TRANSPILER_CACHE_PATH=0 R194_MAP=$S/map.json env -u SEMIO_FLEET_GROUP -u SEMIO_FLEET_NFS_HOST -u SEMIO_FLEET_NETWORK -u SEMIO_REMOTE_CACHE_ENDPOINT -u SEMIO_REMOTE_CACHE_ACCESS -u SEMIO_REMOTE_CACHE_INSTANCE -u SEMIO_REMOTE_CACHE_TOKEN nice -n 19 $BUN test $extra ./🟦️.ts --timeout 180000 2>&1)
  echo "$mode $d: $(echo "$out"|grep -E '^ *[0-9]+ (pass|fail)'|tr -d ' '|tr '\n' ' ')"
  echo "$out"|grep "^(fail)" | sed "s/^/   $mode /" | cut -c1-200
 done
done
echo BATTERY-DONE
