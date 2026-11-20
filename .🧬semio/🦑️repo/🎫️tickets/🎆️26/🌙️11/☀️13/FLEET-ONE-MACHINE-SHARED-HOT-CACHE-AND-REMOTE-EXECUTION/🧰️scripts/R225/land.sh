#!/bin/zsh
# R225 landing: the Nx hash view. New member `📚️library/🏗️build/🧮️task-model/🧿️hash-view` (policy schema, module, law, oracle, corpus) first, then the one-line wiring in `📚️library/🟨️.mjs`. Rehearsal: LAND_ROOT=<scratch root> LAND_LOCKBASE=<scratch> LAND_NOWINDOW=1 LAND_ACK=skip R225_NOTESTS=1 [LAND_FORCE_RED=1] zsh land.sh
HERE=${0:A:h}
R224=${R224_DIR:-$HERE/../R224}
export LAND_STATE=${R225_STATE:?R225_STATE (the prepared state dir)}
export LAND_EXTRA_LOCKS=${LAND_EXTRA_LOCKS-/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️09/SHARED-WORKTREE-FLEET-OVER-NFS/1113-landing.lock}
export LAND_HERE=$R224
. "$R224/land-lib.sh"
BUN=/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f
land_tests(){
 [ -z "$R225_NOTESTS" ] || return 0
 local L=$LAND_ROOT/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library B=$LAND_ROOT/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build out f bad failed=""
 for f in "$B/🧮️task-model/🧿️hash-view/🧪️tests/🟦️.ts" "$B/🧮️task-model/🧭️platform-class/🧪️tests/🟦️.ts" "$B/🧮️task-model/🧭️platform-class/🧪️tests/🧰️nx-e2e/🟦️.ts" "$L/⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts"; do
  cd "$LAND_ROOT" || return 1
  out=$(env -u SEMIO_FLEET_GROUP -u SEMIO_FLEET_NFS_HOST -u SEMIO_FLEET_NETWORK -u SEMIO_REMOTE_CACHE_ENDPOINT -u SEMIO_REMOTE_CACHE_ACCESS -u SEMIO_REMOTE_CACHE_INSTANCE -u SEMIO_REMOTE_CACHE_TOKEN nice -n 19 $BUN test "$f" --timeout 180000 2>&1)
  echo "${f#$LAND_ROOT/}: $(echo "$out" | grep -E '^ *[0-9]+ (pass|fail)' | tr '\n' ' ')"
  bad=$(echo "$out" | grep -E '^\(fail\)'); [ -z "$bad" ] || { echo "$bad" | head -20; failed=1; }
 done
 [ -z "$failed" ] || return 1
 cd "$LAND_ROOT" && $BUN -e 'await import("'"$L"'/🟨️.mjs");await import("'"$B"'/🧮️task-model/🧿️hash-view/🟦️.ts");console.log("import smoke ok")'
}
land_run R225 "nx-hash-view-derived-input-groups-leave-the-project-configuration"
