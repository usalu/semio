#!/bin/zsh
# R226 landing: lease timeouts per class (declared wait, heartbeat, stale-holder rule, typed refusal), ready patterns for the lease wait line. Rehearsal: LAND_ROOT=<scratch root> LAND_LOCKBASE=<scratch> LAND_NOWINDOW=1 LAND_ACK=skip [LAND_FORCE_RED=1] [R226_NOTESTS=1] LAND_EXTRA_LOCKS=
HERE=${0:A:h}
export LAND_STATE=${R226_STATE:-/private/tmp/claude-501/-Users-ueli-Documents-semio/60403654-b0e9-4c0d-a7d3-fba5340a3de9/scratchpad/R226}
export LAND_EXTRA_LOCKS=${LAND_EXTRA_LOCKS-/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️09/SHARED-WORKTREE-FLEET-OVER-NFS/1113-landing.lock}
. "$HERE/../R224/land-lib.sh"
BUN=/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
land_tests(){
 [ -z "$R226_NOTESTS" ] || return 0
 local F=$LAND_ROOT/🧰️framework L=$LAND_ROOT/🧰️framework/🔨️modules/🏃️process/🔒️leases R=$LAND_ROOT/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🛰️sidecar/🚦️ready/🌲️process-tree out p f failed="" row
 for row in "$L/🧪️tests/⏱️class-waits 10 0" "$L/🧪️tests/⌛️timeout-refusal 4 0" "$L/🧭️order/🧪️tests 76 0" "$L/🧭️order/🧪️tests/🕸️wait-for 6 0" "$L/🧭️order/🧪️tests/🪟️registry-root 2 0" "$L/🚪️admission/🧪️tests 17 0" "$L/🪟️windows/🧪️tests 5 0" "$R/🧪️tests 16 0" "$R/🧪️tests/🤝️holder-custody 6 0"; do
  set -- ${=row}; cd "$1" || return 1
  out=$(env -u SEMIO_FLEET_GROUP -u SEMIO_FLEET_NFS_HOST -u SEMIO_FLEET_NETWORK -u SEMIO_REMOTE_CACHE_ENDPOINT -u SEMIO_REMOTE_CACHE_ACCESS -u SEMIO_REMOTE_CACHE_INSTANCE -u SEMIO_REMOTE_CACHE_TOKEN nice -n 19 $BUN test ./🟦️.ts --timeout 180000 2>&1)
  p=$(echo "$out"|grep -E '^ *[0-9]+ pass'|awk '{print $1}'); f=$(echo "$out"|grep -E '^ *[0-9]+ fail'|awk '{print $1}')
  echo "${1#$F/}: pass ${p:-?} fail ${f:-?} (expected $2/$3)"
  { [ "${p:-0}" -ge "$2" ] && [ "${f:-1}" -le "$3" ]; } || { echo "$out" | grep -E '^\(fail\)' | head -10; failed=1; }
 done
 [ -z "$failed" ] || return 1
 cd "$LAND_ROOT" && $BUN -e 'await import("'"$L"'/🟦️.ts");const o=await import("'"$L"'/🧭️order/🟦️.ts");const p=o.leaseOrderPolicyV1();if(o.leaseWaitOfV1(p,"native-fixed-image:x").classId!=="image")process.exit(1);await import("'"$R"'/🟦️.ts");console.log("import smoke ok")'
}
land_run R226 "lease-timeouts-per-class"
