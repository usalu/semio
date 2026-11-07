#!/bin/bash
# usage: chk.sh <label> <manifest> <wasm|native> <cargo check args...>
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
N=$1; M=$2; K=$3; shift 3
O="$C/🗑️generated/1105b/$N"; rm -f "$O.rc"
T=""; [ "$K" = wasm ] && T="--target wasm32-wasip2"
"$(dirname "$0")/slot.sh" "$N" env SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- check --manifest-path "$M" $T --keep-going "$@" > "$O.txt" 2>&1
echo $? > "$O.rc"
