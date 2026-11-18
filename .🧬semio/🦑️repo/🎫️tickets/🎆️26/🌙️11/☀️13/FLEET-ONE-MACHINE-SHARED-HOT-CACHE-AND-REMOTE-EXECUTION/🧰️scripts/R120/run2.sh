#!/bin/bash
# usage: run2.sh <label> <owners-csv|""> [N] [prof]   env: R120_PRE=preload file in this dir, R119_STORE=store dir (unset = private empty store)
cd /Users/ueli/Documents/semio || exit 1
BUN=$PWD/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
D=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/🧰️scripts/R120"
SP=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R120
PRE=(); [ -n "$R120_PRE" ] && PRE=(--preload "$PWD/$D/$R120_PRE")
P=(); [ "$4" = prof ] && P=(--cpu-prof --cpu-prof-dir "$SP/prof-$1")
env -u SEMIO_FLEET_GROUP BUN_JSC_forceRAMSize=${R120_RAM-1073741824} R119_OWNERS="$2" R119_N=${3:-5} nice -n 5 /usr/bin/time -l $BUN "${PRE[@]}" "${P[@]}" "$PWD/$D/accept.ts" > $SP/$1.out 2>&1
echo done >> $SP/$1.out
