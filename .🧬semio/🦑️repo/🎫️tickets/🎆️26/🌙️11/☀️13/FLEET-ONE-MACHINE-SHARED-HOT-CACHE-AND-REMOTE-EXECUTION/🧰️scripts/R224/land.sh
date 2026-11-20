#!/bin/zsh
# R224 landing: the replica applies new files before modifications and holds a modification while a new file of its landing is undelivered; the loader waits for a sibling module the replica has not delivered; the landing helper land-lib.sh. Rehearsal: LAND_ROOT=<scratch root> LAND_LOCKBASE=<scratch> LAND_NOWINDOW=1 LAND_ACK=skip R224_NOTESTS=1 [LAND_FORCE_RED=1] zsh land.sh
HERE=${0:A:h}
export LAND_STATE=${R224_STATE:-/private/tmp/claude-501/-Users-ueli-Documents-semio/60403654-b0e9-4c0d-a7d3-fba5340a3de9/scratchpad/R224/state}
export LAND_EXTRA_LOCKS=${LAND_EXTRA_LOCKS-/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️09/SHARED-WORKTREE-FLEET-OVER-NFS/1113-landing.lock}
. "$HERE/land-lib.sh"
BUN=/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
land_tests(){
 [ -z "$R224_NOTESTS" ] || return 0
 local F=$LAND_ROOT/🧰️framework/🛍️products/🦑️repo/🔨️modules m=$LAND_ROOT/🧰️framework/🛍️products/🦑️repo/🔨️modules out f bad failed=""
 local FS=$F/📚️library/🏗️build/🧷️format-stamps W=$F/🤝️shared-worktree/🔁️replica
 for f in "$W/🧪️tests/🪜️apply-order.ts" "$FS/🧪️tests/📬️delivery-wait/🟦️.ts" "$FS/🔄️reexec/🧪️tests/🟦️.ts" "$FS/🧪️tests/🔁️implementation-edit/🟦️.ts" "$FS/🧪️tests/🟦️.ts" "$W/🧪️tests/🟦️.ts" "$W/🧪️tests/🏁️race.ts" "$W/🧪️tests/🚦️canary.ts" "$W/🧪️tests/⏱️turns.ts"; do
  cd "$(dirname "$f")" || return 1
  out=$(env -u SEMIO_FLEET_GROUP -u SEMIO_FLEET_NFS_HOST -u SEMIO_FLEET_NETWORK -u SEMIO_REMOTE_CACHE_ENDPOINT -u SEMIO_REMOTE_CACHE_ACCESS -u SEMIO_REMOTE_CACHE_INSTANCE -u SEMIO_REMOTE_CACHE_TOKEN nice -n 19 $BUN test ./$(basename "$f") --timeout 180000 2>&1)
  echo "${f#$LAND_ROOT/}: $(echo "$out" | grep -E '^ *[0-9]+ (pass|fail)' | tr '\n' ' ')"
  bad=$(echo "$out" | grep -E '^\(fail\)'); [ -z "$bad" ] || { echo "$bad" | head -20; failed=1; }
 done
 [ -z "$failed" ] || return 1
 cd "$LAND_ROOT" && $BUN -e 'await import("'"$W"'/🟦️.ts");await import("'"$FS"'/🟨️.mjs");await import("'"$FS"'/🔄️reexec/🟨️.mjs");console.log("import smoke ok")'
}
land_run R224 "replica-applies-new-files-first-loader-waits-for-delivery"
