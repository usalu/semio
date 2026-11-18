#!/bin/zsh
# R186: per-file pass/fail counts of the neighbours for the map built by stage.sh. usage: battery.sh <label>
D=$(cd $(dirname $0) && pwd); S=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R186
SC="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🛰️sidecar"; R="$SC/🚦️ready"
BUN=/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
export BUN_RUNTIME_TRANSPILER_CACHE_PATH=0 R186_MAP=$S/map.json R186_PRELOAD=$D/preload.ts; source $S/env.sh
for f in "$R/🌲️process-tree/🧪️tests/🟦️.ts" "$R/🌲️process-tree/🧪️tests/🪟️windows-progress/🟦️.ts" "$R/🌲️process-tree/🧹️containment/🧪️tests/🟦️.ts" "$R/🧪️tests/🎯️probes/🟦️.ts" "$R/🧪️tests/🔁️resume/🟦️.ts" "$R/🧪️tests/🔷️tooling-compiler/🟦️.ts" "$R/🧪️tests/🕰️stale-stage/🟦️.ts" "$R/🧪️tests/🪜️stage-plan/🟦️.ts" "$R/🧪️tests/🟦️.ts" "$SC/🧪️tests/🕸️graph/🟦️.ts" "$SC/🧪️tests/🟦️.ts" "$SC/🧪️tests/🏊️pool/🟦️.ts" "$SC/🧪️tests/🔁️restart/🟦️.ts" "$SC/🧪️tests/🕸️graph/🔭️read/🟦️.ts" "$SC/🧪️tests/🕸️graph/🔬️stall/🟦️.ts"; do
 d=$(dirname "$f"); b=$(basename "$f")
 out=$(cd "$d" && env -u SEMIO_FLEET_GROUP -u SEMIO_FLEET_NFS_HOST -u SEMIO_FLEET_NETWORK -u SEMIO_REMOTE_CACHE_ENDPOINT -u SEMIO_REMOTE_CACHE_ACCESS -u SEMIO_REMOTE_CACHE_INSTANCE -u SEMIO_REMOTE_CACHE_TOKEN nice -n 10 $BUN test --preload $D/preload.ts ./$b --timeout 180000 2>&1)
 echo "$1 ${f#$SC/} $(echo "$out"|/usr/bin/grep -E '^ *[0-9]+ (pass|fail)'|tr -d ' '|tr '\n' ' ') fails: $(echo "$out"|/usr/bin/grep -E '^\(fail\)'|cut -c1-90|tr '\n' '|')"
done
