#!/bin/zsh
# usage: rig.sh <overlay dir> <test dir relative to the root> [file]
S=${0:A:h}
PRE0="/Users/Shared/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/🧰️scripts/R5/lab/overlay-preload.ts"
PRE="${PRELOAD:-$PRE0}"
F=${3:-🟦️.ts}
BUN=/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
mkdir -p $S/sink
export NODE_PATH=/Users/Shared/semio/node_modules R5_OVERLAY=$S/$1 R5_LIVE=/Users/Shared/semio SEMIO_TEST_CHECKOUT=/Users/Shared/semio BUN_RUNTIME_TRANSPILER_CACHE_PATH=0 SEMIO_TEST_ARTIFACT_DIR=$S/sink
D=$S/$1/$2
if [[ -f "$D/$F" ]]; then cp -Rn /Users/Shared/semio/$2/ "$D/" 2>/dev/null; else D=/Users/Shared/semio/$2; fi
cd "$D" && env -u SEMIO_FLEET_GROUP -u SEMIO_FLEET_NFS_HOST -u SEMIO_FLEET_NETWORK -u SEMIO_REMOTE_CACHE_ENDPOINT -u SEMIO_REMOTE_CACHE_ACCESS -u SEMIO_REMOTE_CACHE_INSTANCE -u SEMIO_REMOTE_CACHE_TOKEN $BUN test --preload "$PRE" ./$F --timeout 240000 2>&1
