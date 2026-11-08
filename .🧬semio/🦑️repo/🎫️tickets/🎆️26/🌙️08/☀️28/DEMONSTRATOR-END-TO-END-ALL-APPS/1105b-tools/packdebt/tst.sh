#!/bin/bash
# usage: tst.sh <label> <manifest> <wasm|native> <cargo test args...>
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
N=$1; M=$2; K=$3; shift 3
O="$C/🗑️generated/1105b/$N"; rm -f "$O.rc"
"$C/1105b-tools/slot.sh" "$N" env SEMIO_LANGUAGE=en RUST_BACKTRACE=1 repo cargo --store=shared --configuration=prototype -- test --manifest-path "$M" --no-fail-fast "$@" > "$O.txt" 2>&1
echo $? > "$O.rc"
