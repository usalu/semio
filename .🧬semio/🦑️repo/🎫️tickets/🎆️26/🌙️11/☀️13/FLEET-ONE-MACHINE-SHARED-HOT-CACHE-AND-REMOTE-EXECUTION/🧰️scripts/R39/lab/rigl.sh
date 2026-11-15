#!/bin/zsh
# usage: rigl.sh <overlay or "-" for none> <live test dir relative> [file]
OVL=$1; D2=$2; F=${3:-🟦️.ts}
PRE="/Users/Shared/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/🧰️scripts/R5/lab/overlay-preload.ts"
BUN=/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
export NODE_PATH=/Users/Shared/semio/node_modules R5_LIVE=/Users/Shared/semio SEMIO_TEST_CHECKOUT=/Users/Shared/semio BUN_RUNTIME_TRANSPILER_CACHE_PATH=0
if [[ "$OVL" == "-" ]]; then PL=(); else export R5_OVERLAY=$OVL; PL=(--preload "$PRE"); fi
cd "/Users/Shared/semio/$D2" && env -u SEMIO_FLEET_GROUP -u SEMIO_FLEET_NFS_HOST -u SEMIO_FLEET_NETWORK -u SEMIO_REMOTE_CACHE_ENDPOINT -u SEMIO_REMOTE_CACHE_ACCESS -u SEMIO_REMOTE_CACHE_INSTANCE -u SEMIO_REMOTE_CACHE_TOKEN $BUN test $PL ./$F --timeout 240000 2>&1
