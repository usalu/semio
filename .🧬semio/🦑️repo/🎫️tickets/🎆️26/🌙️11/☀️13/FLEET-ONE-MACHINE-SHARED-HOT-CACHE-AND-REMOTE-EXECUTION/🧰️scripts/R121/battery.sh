#!/bin/zsh
# usage: battery.sh <outfile>   runs LIST through the harness, 4 at a time, one line per file
cd /Users/ueli/Documents/semio || exit 1
D=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/🧰️scripts/R121
source $D/list.sh
BUN=/Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
S=$G/🧪️testing/🌱️genuine-birth/📜️script.ts
one(){ f=$1; for a in 1 2; do out=$(env -u SEMIO_FLEET_GROUP -u SEMIO_FLEET_NFS_HOST -u SEMIO_FLEET_NETWORK -u SEMIO_REMOTE_CACHE_ENDPOINT -u SEMIO_REMOTE_CACHE_ACCESS -u SEMIO_REMOTE_CACHE_INSTANCE -u SEMIO_REMOTE_CACHE_TOKEN $BUN $S --file="$f" --birth=setup --root=/Users/ueli/Documents/semio 2>&1); fail=$(echo "$out" | grep -E "^ +[0-9]+ fail" | tail -1 | tr -s ' ' | tr -d ' '); pass=$(echo "$out" | grep -E "^ +[0-9]+ pass" | tail -1 | tr -s ' ' | tr -d ' '); echo "$fail" | grep -qE "^[1-9][0-9]*fail" || [ -z "$pass" ] || break; done; echo "${f#$BD/} ${pass:-0pass} ${fail:-0fail} a$a"; }
for f in $LIST; do one "$f" & while [ $(jobs -r | wc -l) -ge 4 ]; do sleep 1; done; done > ${1:-/dev/stdout}; wait
