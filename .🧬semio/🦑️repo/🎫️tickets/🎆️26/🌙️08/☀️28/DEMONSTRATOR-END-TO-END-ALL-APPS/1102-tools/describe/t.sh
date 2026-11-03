#!/bin/bash
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"; L=$1; M=$2; shift 2; O="$C/🗑️generated/1102/$L"; rm -f "$O.rc"
SEMIO_LANGUAGE=en bash "$C/1102-tools/slot.sh" $L repo cargo --store=shared --configuration=prototype -- test --manifest-path "$M" --no-fail-fast "$@" > "$O.txt" 2>&1; echo $? > "$O.rc"
