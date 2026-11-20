#!/bin/zsh
# R233 landing: a read follows the owner registry when a peer created, removed or moved an owner during it (settleOwnerRegistryV1); new module + law first, then the schema text and the loader. Rehearsal: LAND_ROOT=<scratch root> LAND_LOCKBASE=<scratch> LAND_NOWINDOW=1 LAND_ACK=skip [LAND_FORCE_RED=1] R233_NOTESTS=1
HERE=${0:A:h}
export LAND_STATE=${R233_STATE:-/private/tmp/claude-501/-Users-ueli-Documents-semio/60403654-b0e9-4c0d-a7d3-fba5340a3de9/scratchpad/R233/state}
export LAND_LIB_DIR=$HERE/../R224
. "$HERE/../R224/land-lib.sh"
BUN=/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
land_tests(){
 [ -z "$R233_NOTESTS" ] || return 0
 local B=$LAND_ROOT/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build d out bad failed=""
 for d in 🧭️registry-follow 🕓️pending-crates 🧲️adopt-registry 🩹️repair-rounds 🌊️seal-under-churn; do
  cd "$B/🧪️tests/🕸️graph/$d" || return 1
  out=$(env -u SEMIO_FLEET_GROUP -u SEMIO_FLEET_NFS_HOST -u SEMIO_FLEET_NETWORK -u SEMIO_REMOTE_CACHE_ENDPOINT -u SEMIO_REMOTE_CACHE_ACCESS -u SEMIO_REMOTE_CACHE_INSTANCE -u SEMIO_REMOTE_CACHE_TOKEN nice -n 19 $BUN test ./🟦️.ts --timeout 180000 2>&1)
  echo "$d: $(echo "$out" | grep -E '^ *[0-9]+ (pass|fail)' | tr '\n' ' ')"
  echo "$out" | grep -qE '^ *[0-9]+ pass' || failed=1
  bad=$(echo "$out" | grep -E '^\(fail\)'); [ -z "$bad" ] || { echo "$bad" | head -20; failed=1; }
 done
 [ -z "$failed" ] || return 1
 cd "$LAND_ROOT" && $BUN -e 'await import("'"$B"'/🕸️graph/🟦️.ts");console.log("import smoke ok")'
}
land_run R233 "graph-read-follows-owner-registry"
