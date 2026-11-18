#!/bin/zsh
# usage: battery.sh <outfile> <base|stage> <listfile> [parallel]   one line per file: name Npass Mfail aN
BUN=/Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
D=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/🧰️scripts/R122
cd /Users/ueli/Documents/semio || exit 1
out=$1; mode=$2; list=$3; par=${4:-4}
one(){ f=$1; P=(); [ "$mode" = stage ] && P=(--preload $D/preload3.ts); for a in 1 2; do o=$(cd "$(dirname "/Users/ueli/Documents/semio/$f")" && env -u SEMIO_FLEET_GROUP -u SEMIO_FLEET_NFS_HOST -u SEMIO_FLEET_NETWORK -u SEMIO_REMOTE_CACHE_ENDPOINT -u SEMIO_REMOTE_CACHE_ACCESS -u SEMIO_REMOTE_CACHE_INSTANCE -u SEMIO_REMOTE_CACHE_TOKEN SEMIO_TEST_ARTIFACT_DIR=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R122/artifacts $BUN test $P ./$(basename "$f") --timeout 180000 2>&1); fail=$(echo "$o" | grep -E "^ +[0-9]+ fail" | tail -1 | tr -d ' '); pass=$(echo "$o" | grep -E "^ +[0-9]+ pass" | tail -1 | tr -d ' '); echo "$fail" | grep -qE "^[1-9][0-9]*fail" || [ -z "$pass" ] || break; done; echo "$f ${pass:-0pass} ${fail:-0fail} a$a"; }
while read f; do one "$f" & while [ $(jobs -r | wc -l) -ge $par ]; do sleep 1; done; done < $list > $out; wait
