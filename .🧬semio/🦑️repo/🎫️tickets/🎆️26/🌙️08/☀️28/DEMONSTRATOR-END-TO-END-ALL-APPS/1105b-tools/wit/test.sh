#!/bin/bash
# usage: test.sh <label> <manifest> <cargo test args...>
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
N=$1; M=$2; shift 2
O="$C/🗑️generated/1105b/$N"; rm -f "$O.rc"
"$C/1105b-tools/slot.sh" "$N" env SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- test --manifest-path "$M" --no-fail-fast "$@" > "$O.txt" 2>&1
echo $? > "$O.rc"
