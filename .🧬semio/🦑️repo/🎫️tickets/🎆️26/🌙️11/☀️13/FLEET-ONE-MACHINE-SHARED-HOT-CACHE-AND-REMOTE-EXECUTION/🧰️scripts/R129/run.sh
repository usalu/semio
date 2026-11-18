#!/bin/bash
# usage: run.sh <label> <owners-csv> [gc]  env R129_CODE
cd /Users/ueli/Documents/semio || exit 1
BUN=$PWD/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
D=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/🧰️scripts/R129"
SP=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R129; mkdir -p $SP
env -u SEMIO_FLEET_GROUP BUN_RUNTIME_TRANSPILER_CACHE_PATH=0 BUN_JSC_forceRAMSize=${R129_RAM-1207959552} MIMALLOC_PURGE_DELAY=${R129_PURGE-0} BUN_JSC_numberOfGCMarkers=1 BUN_JSC_useConcurrentGC=0 R129_VMMAP=${R129_VMMAP:-0} R129_RELEASE=${R129_RELEASE:-0} R129_GC=${3:-0} R129_OWNERS="$2" nice -n 5 /usr/bin/time -l $BUN ${R129_PRE:+--preload "$PWD/$D/$R129_PRE"} "$PWD/$D/measure.ts" > $SP/$1.out 2>&1
echo done >> $SP/$1.out
