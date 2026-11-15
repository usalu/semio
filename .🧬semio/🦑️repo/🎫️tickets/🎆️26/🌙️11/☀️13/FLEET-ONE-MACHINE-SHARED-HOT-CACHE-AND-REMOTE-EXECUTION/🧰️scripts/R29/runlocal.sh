#!/bin/zsh
# usage: runlocal.sh <overlay name> <test dir relative to the root> [file]; runs the test file from the overlay copy when the overlay holds one, else from the live location, with the overlay redirects
HERE=${0:A:h};S=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R29
F=${3:-🟦️.ts}
BUN=/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
export R5_OVERLAY=$S/$1 R5_LIVE=/Users/Shared/semio SEMIO_TEST_CHECKOUT=/Users/Shared/semio BUN_RUNTIME_TRANSPILER_CACHE_PATH=0
D=$S/$1/$2; if [[ -f "$D/$F" ]]; then cp -Rn /Users/Shared/semio/$2/ "$D/" 2>/dev/null; for x in "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🗓️scheduling/🔣️.json" "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🔌️providers/🧬️schema/🔣️.json"; do mkdir -p "$S/$1/${x:h}"; cp -n "/Users/Shared/semio/$x" "$S/$1/$x" 2>/dev/null; done; else D=/Users/Shared/semio/$2; fi
cd "$D" && env -u SEMIO_FLEET_GROUP -u SEMIO_FLEET_NFS_HOST -u SEMIO_FLEET_NETWORK -u SEMIO_REMOTE_CACHE_ENDPOINT -u SEMIO_REMOTE_CACHE_ACCESS -u SEMIO_REMOTE_CACHE_INSTANCE -u SEMIO_REMOTE_CACHE_TOKEN $BUN test --preload $HERE/../R5/lab/overlay-preload.ts ./$F --timeout 240000 2>&1
