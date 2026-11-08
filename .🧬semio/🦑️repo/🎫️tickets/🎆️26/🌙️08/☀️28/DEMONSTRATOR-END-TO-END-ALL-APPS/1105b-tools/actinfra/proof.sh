#!/bin/bash
# usage: proof.sh <N> <base> [manifest package]  waits for lock-sync-1107-ok, builds through a W- slot, then runs the stage drivers
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"; G="$C/🗑️generated/1105b"; A="$C/1105b-tools/actinfra"
N="$1"; B="$2"; shift 2; R="$G/actinfra-proof-$N"; rm -f "$R.rc"
while [ ! -f "$G/lock-sync-1107-ok" ]; do sleep 30; done
bash "$A/build.sh" "$N" "$@"; rc=$(cat "$G/actinfra-build-$N.rc")
[ "$rc" = 0 ] || { echo "build rc $rc" > "$R.txt"; echo "$rc" > "$R.rc"; exit 0; }
W=$(ls -t .cache/b/r/m/u/l/*/t*/wasm32-wasip2/prototype/$B.wasm 2>/dev/null | head -1)
[ -n "$W" ] || { echo "no wasm $B" > "$R.txt"; echo 3 > "$R.rc"; exit 0; }
O="$G/actinfra-stages-$N"; { echo "wasm $W $(stat -f %z "$W")"; bash "$A/stages.sh" "$W" "$O" && STAGE_SCRIPT=stage3.ts bash "$A/stages.sh" "$O"; } > "$R.txt" 2>&1
echo $? > "$R.rc"
